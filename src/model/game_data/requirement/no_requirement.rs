use crate::model::game_data::GameData;
use crate::model::game_data::requirement::Requirement;

pub struct NoRequirement;

impl Requirement for NoRequirement {
    fn is_fulfilled(&self, _: &GameData) -> bool {
        true
    }
}