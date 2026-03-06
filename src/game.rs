#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    ChoosingPayoo, // First player of the round chooses Payoo suit
    PlayerTurn,    // Human picks a card to play
    AITurn,        // AI plays automatically
    TrickEnd,      // Show trick result briefly before next trick
    RoundEnd,      // Show round scores
    GameOver,      // Final scores
}
