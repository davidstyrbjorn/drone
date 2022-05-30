use crate::prelude::*;

// System which converts turn state to something better
#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(TelerportationCrystal)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    // Get the teleportation crystal point
    let mut crystal = <&Point>::query().filter(component::<TelerportationCrystal>());
    let crystal_default = Point::new(-1, -1);
    // Unwrap-or will not crash like unwrap but will return None if it fails
    // The argument to unwrap_or is the default that will be used if None is returned
    let teleportation_pos = crystal.iter(ecs).nth(0).unwrap_or(&crystal_default);

    let current_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    // Get the player_hp component
    let mut player_hp_and_pos = <(&Health, &Point)>::query().filter(component::<Player>());
    // Did the player die?
    player_hp_and_pos.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if pos == teleportation_pos {
            new_state = TurnState::Victory;
        }
        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
    });

    *turn_state = new_state;
}
