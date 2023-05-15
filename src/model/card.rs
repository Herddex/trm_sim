mod card_builder;
pub(crate) mod card_compendium;

use crate::model::game_data::GameData;
use crate::model::game_data::mutation::normal_mutation::NormalMutation;
use crate::model::game_data::mutation::normal_mutation::NormalMutation::CompositeMutation;
use crate::model::game_data::requirement::{Requirement};

pub type CardId = usize;

pub struct Card {
    mutation: NormalMutation,
    requirement: Option<Requirement>,
}

impl Card {
    pub fn new(id: CardId, mut immediate_effects: Vec<NormalMutation>, requirement: Option<Requirement>) -> Self {
        immediate_effects.insert(0, NormalMutation::CardPlayMutation(id));
        Self {
            mutation: CompositeMutation(immediate_effects),
            requirement,
        }
    }

    pub fn play(&self, game_data: &mut GameData) -> Result<(), ()> {
        if let Some(requirement) = &self.requirement {
            if !requirement.is_fulfilled(game_data) {
                return Err(());
            }
        }
        self.mutation.apply(game_data)
    }
}