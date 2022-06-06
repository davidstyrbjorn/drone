use crate::prelude::*;

#[system]
#[write_component(Stunned)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    // Query for all the lasting effect components and put them into effect or delife them
    let mut stunned = <(Entity, &mut Stunned)>::query();

    stunned.iter_mut(ecs).for_each(|(e, s)| {
        s.0 -= 1;
        if s.0 <= 0 {
            commands.remove_component::<Stunned>(*e);
        }
    });
}
