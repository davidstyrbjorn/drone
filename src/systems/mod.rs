use crate::prelude::*;

mod collision;
mod end_turn;
mod entity_render;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;

// Dividing the Scheduler
/*
* It doesn't make sense to run all the system at all times
* Nothing can move if the state is AwaitingInput for instnace
* We can restrain which systems run in each phase by creating seperate schedulers for each turn state
*/

// A schedule - execution plan for our systems

// While awaiting input
// the screen still needs to display the map and entities. It also calls the player_input system.
pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .build()
}

// When it’s the player’s turn, the game doesn’t accept input—but does check for collisions
// as well as rendering everything. It finishes with end_turn.
pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collision::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

// The monsters’ turn is very similar to the player’s, but adds random movement.
pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
