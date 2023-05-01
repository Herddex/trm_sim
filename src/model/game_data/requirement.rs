use crate::model::game_data::GameData;
use crate::model::tag::Tag;

pub trait Requirement {
    fn is_fulfilled(&self, game_data: &GameData) -> bool;
}

pub struct NoRequirement;
impl Requirement for NoRequirement {
    fn is_fulfilled(&self, _: &GameData) -> bool {
        true
    }
}

pub struct MinOxygenRequirement(pub i32);
impl Requirement for MinOxygenRequirement {
    fn is_fulfilled(&self, game_data: &GameData) -> bool {
        game_data.oxygen >= self.0
    }
}

pub struct MaxOxygenRequirement(pub i32);
impl Requirement for MaxOxygenRequirement {
    fn is_fulfilled(&self, game_data: &GameData) -> bool {
        game_data.oxygen <= self.0
    }
}

pub struct MinTemperatureRequirement(pub i32);
impl Requirement for MinTemperatureRequirement {
    fn is_fulfilled(&self, game_data: &GameData) -> bool {
        game_data.temperature >= self.0
    }
}

pub struct MaxTemperatureRequirement(pub i32);
impl Requirement for MaxTemperatureRequirement {
    fn is_fulfilled(&self, game_data: &GameData) -> bool {
        game_data.temperature <= self.0
    }
}

pub struct MinOceanRequirement(pub usize);
impl Requirement for MinOceanRequirement {
    fn is_fulfilled(&self, game_data: &GameData) -> bool {
        game_data.board.placed_oceans() >= self.0
    }
}

pub struct MaxOceanRequirement(pub usize);
impl Requirement for MaxOceanRequirement {
    fn is_fulfilled(&self, game_data: &GameData) -> bool {
        game_data.board.placed_oceans() <= self.0
    }
}

pub struct TagRequirement(pub Tag, pub i32);
impl Requirement for TagRequirement {
    fn is_fulfilled(&self, game_data: &GameData) -> bool {
        *game_data.tags.get(&self.0).unwrap() >= self.1
    }
}