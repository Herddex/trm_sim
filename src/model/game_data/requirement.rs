mod no_requirement;

use crate::model::game_data::GameData;

pub trait Requirement {
    fn is_fulfilled(&self, game_data: &GameData) -> bool;
}