use libsts::{Card, Save};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    pub cards: Vec<Card>,
    pub relics: Vec<String>,
    pub key: String,
}

impl UpdateRequest {
    pub fn new(save: &Save, key: String) -> Self {
        Self {
            cards: save.cards.clone(),
            relics: save.relics.clone(),
            key,
        }
    }
}
