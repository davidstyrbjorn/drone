use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut views = <(&Point, &mut FieldOfView)>::query();
    // Go through each component that has field of view & point and set fov.visible_tiles to something according
    // Our Map has implemented Algorithm2D so we can use field_of_view_set to get a HashSet of visible tiles
    views
        .iter_mut(ecs)
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
            fov.is_dirty = false;
        });
}
