use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(Stunned)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                // If it's a player tell the camera we moved and give the camera our new position
                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
            // If want_move.entity has a Stunned, don't move
            if entry.get_component::<Stunned>().is_err() {
                // By adding a new point we replace the existing one
                // This is faster since it lets Legion delegate the command instead of us assigning the data ourselves
                commands.add_component(want_move.entity, want_move.destination);
            }
        }
    }

    // We have handled this message
    commands.remove(*entity);
}
