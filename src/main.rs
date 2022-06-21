use std::collections::HashSet;

// Brings modules into scope
mod camera;
mod components;
mod event_log;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::event_log::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    // Collection of entities + components
    ecs: World,
    resources: Resources, // Shared data
    // Various system collections
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        // Creates a map builder from which we grab our map
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng, 0);
        spawn_player(&mut ecs, map_builder.player_start);
        // For the love of god, SEAL THE EXITS - krieger
        let exit_idx = map_builder
            .map
            .point2d_to_index(map_builder.teleportation_crystal_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(
            &mut ecs,
            &mut rng,
            0,
            &map_builder.monster_spawns,
            &map_builder.guaranteed_monster_spawns,
        );

        // Inject our map and camera as resources (since this is what is shared in our program)
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::Menu);
        resources.insert(map_builder.theme);
        resources.insert(EventLog::new());

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn menu(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);

        ctx.print_centered(40, "DRONE");
        ctx.print_centered(45, "PRESS ANY BUTTON TO CRASH LAND ON EARTH");
        ctx.print_right(
            SCREEN_WIDTH * 2,
            (SCREEN_HEIGHT * 2) - 5,
            "BY: DAVID STYRBJÖRN",
        );
        ctx.print_right(
            SCREEN_WIDTH * 2,
            (SCREEN_HEIGHT * 2) - 3,
            "ART: EMIL BERTHOLDSSON",
        );
        ctx.print_right(
            SCREEN_WIDTH * 2,
            (SCREEN_HEIGHT * 2) - 1,
            "BALANCE HELP: MAX BENECKE",
        );

        // Check if the user has pressed any key
        if let Some(_) = ctx.key {
            self.resources.insert(TurnState::AwaitingInput);
        }
    }

    fn reset_game_state(&mut self) {
        // Reset legion stuff and other variables!
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng, 0);
        // Spawn in entities
        spawn_player(&mut self.ecs, map_builder.player_start);
        // For the love of god, SEAL THE EXITS - krieger
        // spawn_telerportation_crystal(&mut self.ecs, map_builder.teleportation_crystal_start);
        let exit_idx = map_builder
            .map
            .point2d_to_index(map_builder.teleportation_crystal_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        // Spawn monsters and items
        spawn_level(
            &mut self.ecs,
            &mut rng,
            0,
            &map_builder.monster_spawns,
            &map_builder.guaranteed_monster_spawns,
        );

        // Insert resources into ecs system
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
        self.resources.insert(EventLog::new());
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "...the drone has crashed...");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "DESTROYED by a monoster, your drone's journey has prematurely ended",
        );
        ctx.print_color_centered(5, WHITE, BLACK, "The telerportation crystal remains not found so this drone did not make it home to its fellow drones.");
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Be not frail, you can fly in with another drone and try again.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press R to fly again");

        // Check for reset input
        if let Some(VirtualKeyCode::R) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "The drone could teleport home! The drone can now drone forever!!",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press R to help another drone");

        // Check for reset input
        if let Some(VirtualKeyCode::R) = ctx.key {
            self.reset_game_state();
        }
    }

    fn advance_level(&mut self) {
        // Get the player entity id
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .nth(0)
            .unwrap();

        // Create a Set of entities to not kill, insert player
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        // Make sure all the carried items are kept to next level
        <(Entity, &Carried)>::query()
            .iter(&mut self.ecs)
            .filter(|(_, carry)| carry.0 == player_entity)
            .map(|(e, _)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        // A much more effiecent way of performing multiple commands to the ECS system
        // is through this method
        let mut command_buffer = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            // Will iterate over EVERY entity in the world
            // Do we keep it or remove it?
            if !entities_to_keep.contains(e) {
                command_buffer.remove(*e);
            }
        }

        // Mark field of view as dirty
        // Making it not retain to the next level
        // Notice the iter_mut(...) here since we modify the fov component
        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        // Create a map just like we've done in other functions before
        let mut rng = RandomNumberGenerator::new();
        // Get the player and thus map level
        let mut mb = MapBuilder::new(
            &mut rng,
            self.ecs
                .entry_ref(player_entity)
                .unwrap()
                .get_component::<Player>()
                .unwrap()
                .map_level
                + 1,
        );

        // Calculate new map level and start pos
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = mb.player_start.x;
                pos.y = mb.player_start.y;
            });

        // Decide on wheter we spawn staircase or teleportation crystal
        if map_level == 4 {
            spawn_telerportation_crystal(&mut self.ecs, mb.teleportation_crystal_start);
        } else {
            let exit_idx = mb.map.point2d_to_index(mb.teleportation_crystal_start);
            mb.map.tiles[exit_idx] = TileType::Exit;
        }

        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &mb.monster_spawns,
            &mb.guaranteed_monster_spawns,
        );

        // Finally add our ECS resources as always
        self.resources.insert(mb.map);
        self.resources.insert(Camera::new(mb.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(mb.theme);
        self.resources.insert(EventLog::new());
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clear all layers
        ctx.set_active_console(0);
        // Now grab mouse position and insert as resource
        // There is a Point::from_tuple(...) but i wanted to try enum accessor pattern here
        self.resources
            .insert(Point::new(ctx.mouse_pos().0, ctx.mouse_pos().1));
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();

        // No need to worry about dupliactes, when we insert a resource of the same type
        // It replaces if there is an already existing resource of the same type!
        self.resources.insert(ctx.key);

        // Execute systems, mutable borrow form ecs and resources!
        // Depending on TurnState we execute different scheduler
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::Menu => self.menu(ctx),
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
        }

        // Render draw buffer
        render_draw_buffer(ctx).expect("Render error!");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Drone in a Dungeon")
        .with_fps_cap(144.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
