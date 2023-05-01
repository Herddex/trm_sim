mod card_builder;
mod card_compendium;

use crate::model::game_data::mutation::NormalMutation;
use crate::model::game_data::requirement::Requirement;

pub(crate) type CardId = usize;

pub struct Card {
    requirement: Box<dyn Requirement>,
    immediate_effects: NormalMutation
}