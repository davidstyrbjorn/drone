use crate::prelude::*;

// System which converts turn state to something better
#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(TelerportationCrystal)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    // Get the player_hp component
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    // Get the teleportation crystal point
    let mut amulet = <&Point>::query().filter(component::<TelerportationCrystal>());
    let teleportation_pos = amulet.iter(ecs).nth(0).unwrap();
    let current_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    // Did the player die?
    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if pos == teleportation_pos {
            new_state = TurnState::Victory;
        }
    });

    *turn_state = new_state;
}
