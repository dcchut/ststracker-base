use libsts::Card;
use serde::{Deserialize, Serialize};

/// A client request to update their card / relic list
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct UpdateRequest {
    /// The game state of the client
    #[serde(flatten)]
    game_state: GameState,

    /// The client API key
    pub key: String,
}

impl UpdateRequest {
    /// Returns a new update request with the given cards, relics, and key
    ///
    /// # Example
    /// ```rust
    /// use ststracker_base::api::{GameState, UpdateRequest};
    ///
    /// let game_state = GameState::default();
    /// let update_request = UpdateRequest::new(game_state, "my secret key");
    /// ```
    pub fn new(game_state: GameState, key: &str) -> Self {
        Self {
            game_state,
            key: String::from(key),
        }
    }

    pub fn cards(&self) -> &[Card] {
        &self.game_state.cards
    }

    pub fn relics(&self) -> &[String] {
        &self.game_state.relics
    }

    pub fn game_state(&self) -> &GameState { &self.game_state }
}

/// A potential state of our STS game
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct GameState {
    /// The cards in the response
    pub cards: Vec<Card>,

    /// The relics in the response
    pub relics: Vec<String>,
}

impl GameState {
    /// Returns a struct representing the state of the game
    ///
    /// # Example
    /// ```rust
    /// use ststracker_base::api::GameState;
    ///
    /// let game_state = GameState::new(Vec::new(),
    ///                                   Vec::new());
    /// ```
    pub fn new(cards: Vec<Card>, relics: Vec<String>) -> Self {
        Self { cards, relics }
    }
}
