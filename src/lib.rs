use libsts::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    pub cards: Vec<Card>,
    pub relics: Vec<String>,
    pub key: String,
}

impl UpdateRequest {
    pub fn from_get_request(get_request : GetRequest, key : &str) -> Self {
        Self {
            cards : get_request.cards,
            relics : get_request.relics,
            key : String::from(key),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub cards: Vec<Card>,
    pub relics: Vec<String>,
}

impl GetRequest {
    pub fn new(cards: Vec<Card>, relics: Vec<String>) -> Self {
        Self { cards, relics }
    }
}
