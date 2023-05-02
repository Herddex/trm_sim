use crate::model::card::CardId;
use crate::model::game_data::GameData;

enum Action {
    Card(CardId)
}

impl Action {
    pub fn execute(game_data: &mut GameData) -> Result<(), ()> {
        todo!()
    }
}