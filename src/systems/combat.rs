use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Get our list of attackers
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect::<Vec<(Entity, Entity)>>();
    // We don't mofidy the attack victim inside the query since that could lead to borrow checker problems
    // The rust way is usually to obtain the list of data and then modify in a seperate loop
    victims.iter().for_each(|(message, victim)| {
        // Does our victim have a Health component
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*victim);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    });
}
