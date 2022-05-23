mod camera;
mod components;
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
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        // Spawn monsters in each room
        map_builder
            .rooms
            .iter()
            .skip(1) // The player room is skipped
            .map(|r| r.center()) // Grab each rectangle center
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        // Inject our map and camera as resources (since this is what is shared in our program)
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
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
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MonsterTurn => {
                self.monster_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
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
