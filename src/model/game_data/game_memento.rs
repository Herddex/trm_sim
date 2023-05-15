use crate::model::game_data::GameData;

pub struct GameMemento {
    game_data: GameData
}

impl GameMemento {
    pub fn new(game_data: GameData) -> Self {
        Self {
            game_data
        }
    }

    pub fn to_game(self) -> GameData {
        self.game_data
    }
}