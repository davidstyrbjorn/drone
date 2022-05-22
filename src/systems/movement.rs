use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // By adding a new point we replace the existing one
        // This is faster since it lets Legion delegate the command instead of us assigning the data ourselves
        commands.add_component(want_move.entity, want_move.destination);

        // If it's a player tell the camera we moved and give the camera our new position
        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.destination);
        }
    }

    // We have handled this message
    commands.remove(*entity);
}
