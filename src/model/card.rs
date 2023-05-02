mod card_builder;
mod card_compendium;

use crate::model::game_data::GameData;
use crate::model::game_data::mutation::{Mutation};
use crate::model::game_data::requirement::{NewRequirement};

pub(crate) type CardId = usize;

pub struct Card {
    immediate_effects: Mutation,
    requirement: Option<NewRequirement>
}

impl Card {
    pub fn play(&self, game_data: &mut GameData) -> Result<(), ()> {
        if let Some(requirement) = &self.requirement {
            if !requirement.is_fulfilled(game_data) {
                return Err(());
            }
        }
        game_data.mutate(&self.immediate_effects)
    }
}