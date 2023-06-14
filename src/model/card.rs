mod card_builder;
pub(crate) mod card_compendium;
pub(crate) mod requirement;

use crate::action::invalid_action::{ActionResult, InvalidActionError};
use crate::model::card::requirement::Requirement;
use crate::model::game::mutation::Mutation;
use crate::model::game::Game;
use std::fmt::{Display, Formatter};

pub type CardId = i32;

pub(crate) struct Card {
    mutation: Mutation,
    requirement: Option<Requirement>,
}

impl Card {
    pub(crate) fn new(mutation: Mutation, requirement: Option<Requirement>) -> Self {
        Self {
            mutation,
            requirement,
        }
    }

    pub(crate) fn play(&self, game: &mut Game) -> ActionResult {
        if let Some(requirement) = &self.requirement {
            if !requirement.is_fulfilled(game) {
                return Err(InvalidActionError::new(format!(
                    "Requirement not fulfilled: {}",
                    requirement
                )));
            }
        }
        self.mutation.apply(game)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mutation)?;
        if let Some(ref requirement) = self.requirement {
            writeln!(f, "Requirement: {}", requirement)?;
        }
        Ok(())
    }
}
