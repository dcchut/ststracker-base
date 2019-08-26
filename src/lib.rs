pub mod api;

#[cfg(test)]
mod tests {
    use super::*;
    use libsts::Card;

    #[test]
    fn test_update_request_flatten() {
        let cards = vec![
            Card {
                id: String::from("TestCard"),
                misc: 0,
                upgrades: 0,
            },
            Card {
                id: String::from("TestCard2"),
                misc: 0,
                upgrades: 1,
            },
        ];

        let relics = vec![String::from("TestRelic1"), String::from("TestRelic2")];

        let game_state = api::GameState::new(cards, relics);
        let update_request = api::UpdateRequest::new(game_state, "my secret key");

        let json =
            serde_json::to_string(&update_request).expect("failed to serialize update request");

        assert_eq!(json, r#"{"cards":[{"id":"TestCard","misc":0,"upgrades":0},{"id":"TestCard2","misc":0,"upgrades":1}],"relics":["TestRelic1","TestRelic2"],"key":"my secret key"}"#);
    }
}
