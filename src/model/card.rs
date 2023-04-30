mod card_builder;

use crate::model::game_data::mutation::NormalMutation;
use crate::model::game_data::requirement::Requirement;
use crate::model::tag::Tag;

pub(crate) type CardId = usize;

pub struct Card {
    cost: i32,
    requirement: Box<dyn Requirement>,
    tags: Vec<Tag>,
    victory_points: i32,
    immediate_effects: NormalMutation
}