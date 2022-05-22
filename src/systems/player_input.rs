// Nested module
// There will be some proc macros with alot of boilerplate code, ignore this for now
// as this is a very advanced Rust feature

use crate::prelude::*;

#[system]
// These decorators help Legion know which data we are planning on reading/writing to
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = *key {
        // Get our movement vector
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            // Construct a query grabbing all Player & Point components combination in our world
            players.iter(ecs).for_each(|(entity, pos)| {
                let destination = *pos + delta;
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            });
            *turn_state = TurnState::PlayerTurn;
        }
    }
}
