#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    Menu,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}
