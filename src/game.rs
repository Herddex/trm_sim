use crate::game::action::Action;
use crate::model::game_data::board::game_map::GameMap;
use crate::model::game_data::GameData;
use crate::model::game_data::game_memento::GameMemento;

pub mod action;

pub struct Game {
    game_data: GameData,
    history: Vec<GameMemento>
}

impl Game {
    pub fn new(game_map: &'static GameMap) -> Self {
        Game {
            game_data: GameData::new(game_map),
            history: Vec::new()
        }
    }

    pub fn do_action(&mut self, action: Action) -> Result<(), ()> {
        self.history.push(GameMemento::new(self.game_data.clone()));
        if let Err(_) = action.execute(&mut self.game_data) {
            self.game_data = self.history.pop().unwrap().to_game();
            Err(())
        } else {
            Ok(())
        }
    }
}