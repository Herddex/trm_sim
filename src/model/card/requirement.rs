use crate::model::game::Game;
use crate::model::tag::Tag;
use std::fmt::{Display, Formatter};

pub(crate) enum Requirement {
    MinOxygen(i32),
    MaxOxygen(i32),
    MinTemperature(i32),
    MaxTemperature(i32),
    MinOceanCount(i32),
    MaxOceanCount(i32),
    Tag(Tag, i32),
}

impl Requirement {
    pub(crate) fn is_fulfilled(&self, game: &Game) -> bool {
        match self {
            Self::MinOxygen(amount) => game.oxygen() >= *amount,
            Self::MaxOxygen(amount) => game.oxygen() <= *amount,
            Self::MinTemperature(amount) => game.temperature() >= *amount,
            Self::MaxTemperature(amount) => game.temperature() <= *amount,
            Self::MinOceanCount(amount) => game.oceans() >= *amount,
            Self::MaxOceanCount(amount) => game.oceans() <= *amount,
            Self::Tag(tag, amount) => game.tag(*tag) >= *amount,
        }
    }
}

impl Display for Requirement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MinOxygen(amount) => write!(f, "At least {}% oxygen", amount),
            Self::MaxOxygen(amount) => write!(f, "At most {}% oxygen", amount),
            Self::MinTemperature(amount) => write!(f, "At least {} degrees C", amount),
            Self::MaxTemperature(amount) => write!(f, "At most {} degrees C", amount),
            Self::MinOceanCount(amount) => write!(f, "At least {} oceans in play", amount),
            Self::MaxOceanCount(amount) => write!(f, "At most {} oceans in play", amount),
            Self::Tag(tag, amount) => write!(f, "At least {} {} tags", amount, tag),
        }
    }
}
