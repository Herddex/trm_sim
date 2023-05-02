use crate::model::game_data::GameData;
use crate::model::tag::Tag;

pub enum NewRequirement {
    MinOxygen(i32),
    MaxOxygen(i32),
    MinTemperature(i32),
    MaxTemperature(i32),
    MinOcean(usize),
    MaxOcean(usize),
    Tag(Tag, i32)
}

impl NewRequirement {
    pub fn is_fulfilled(&self, game_data: &GameData) -> bool {
        match self {
            Self::MinOxygen(amount) => game_data.oxygen >= *amount,
            Self::MaxOxygen(amount) => game_data.oxygen <= *amount,
            Self::MinTemperature(amount) => game_data.temperature >= *amount,
            Self::MaxTemperature(amount) => game_data.temperature <= *amount,
            Self::MinOcean(amount) => game_data.board.placed_oceans() >= *amount,
            Self::MaxOcean(amount) => game_data.board.placed_oceans() <= *amount,
            Self::Tag(tag, amount) => *game_data.tags.get(tag)
                .expect("Tag should be in the map") >= *amount
        }
    }
}