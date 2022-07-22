use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
#[read_component(Stunned)]
#[read_component(Name)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Get our list of attackers
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    // We don't mofidy the attack victim inside the query since that could lead to borrow checker problems
    // The rust way is usually to obtain the list of data and then modify in a seperate loop
    victims.iter().for_each(|(message, attacker, victim)| {
        // Does the attacker have a stunned component?
        let is_attacker_stunned = ecs
            .entry_ref(*attacker)
            .unwrap()
            .get_component::<Stunned>()
            .is_ok();

        if !is_attacker_stunned {
            // Calculate the base damage from our attacker
            let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
                if let Ok(dmg) = v.get_component::<Damage>() {
                    dmg.0
                } else {
                    0
                }
            } else {
                0
            };

            // get weapon damage from attacker
            // Query for all Carried + Damage components and then sum them up for total weapon damage on our holder
            let weapon_damage: i32 = <(&Carried, &Damage)>::query()
                .iter(ecs)
                .filter(|(carried, _)| carried.0 == *attacker)
                .map(|(_, dmg)| dmg.0)
                .sum();

            let victim_is_player = ecs
                .entry_ref(*victim)
                .unwrap()
                .get_component::<Player>()
                .is_ok();

            let attacker_is_player = ecs
                .entry_ref(*attacker)
                .unwrap()
                .get_component::<Player>()
                .is_ok();

            // Is our attacker the player and does the enemy have a name?
            // Log it to event
            if attacker_is_player {
                if let Ok(name) = ecs.entry_ref(*victim).unwrap().get_component::<Name>() {
                    EventLog::log(
                        commands,
                        format!(
                            "Player attacked {} for {} damage",
                            name.0,
                            base_damage + weapon_damage
                        ),
                    );
                }
            }

            // Does our victim have a Health component
            if let Ok(mut health) = ecs
                .entry_mut(*victim)
                .unwrap()
                .get_component_mut::<Health>()
            {
                let final_damage = base_damage + weapon_damage;

                health.current -= final_damage;
                if health.current < 1 && !victim_is_player {
                    commands.remove(*victim);
                }
            }
        }

        commands.remove(*message);
    });
}
