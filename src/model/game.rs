use std::collections::HashMap;
use crate::model::board::Board;
use crate::model::card::{Card, Tag};
use crate::model::resource::{Production, Resource};

const MAX_TEMPERATURE: i32 = 8;
const MAX_OXYGEN: i32 = 14;

#[derive(Clone)]
pub struct Game {
    tr: i32,

    oxygen: i32,
    temperature: i32,
    oceans: i32,

    resources: HashMap<Resource, i32>,
    productions: HashMap<Production, i32>,

    board: Board,

    tags: HashMap<Tag, i32>,

    cards_in_hand: Vec<Card>,
    played_cards: Vec<Card>,
    cards_to_be_drawn: Vec<Card>,

    victory_points: i32,
}

pub enum Mutation {
    CompositeMutation(Vec<Mutation>),
    ProductionMutation(Production, i32),
    ResourceMutation(Resource, i32),
    TrMutation(i32),
    TemperatureIncreaseMutation,
    OxygenIncreaseMutation,
    VictoryPointMutation(i32),
}

impl Game {
    pub fn mutate(&mut self, mutation: &Mutation) -> Result<(), ()> {
        let clone = self.clone();
        match self.mutate_internal(mutation) {
            Ok(_) => Ok(()),
            Err(_) => {
                *self = clone;
                Err(())
            }
        }
    }

    fn mutate_internal(&mut self, mutation: &Mutation) -> Result<(), ()> {
        match mutation {
            Mutation::CompositeMutation(mutations) => {
                for mutation in mutations {
                    self.mutate_internal(mutation)?;
                }
            }
            Mutation::ProductionMutation(production, amount) => {
                let production_value = self.productions
                    .get_mut(production)
                    .expect("Production should be initialized in Game struct");
                *production_value += amount;
                if *production_value < production.minimum_value() { return Err(()); }
            }
            Mutation::ResourceMutation(resource, amount) => {
                let resource_value = self.resources
                    .get_mut(resource)
                    .expect("Resource should be initialized in Game struct");
                *resource_value -= amount;
                if *resource_value < 0 { return Err(()); }
            }
            Mutation::TrMutation(amount) => self.tr += amount,
            Mutation::TemperatureIncreaseMutation => {
                if self.temperature < MAX_TEMPERATURE {
                    self.temperature += 2;
                    self.tr += 1;
                }
            }
            Mutation::OxygenIncreaseMutation => {
                if self.oxygen < MAX_OXYGEN {
                    self.oxygen += 1;
                    self.tr += 1;
                }
            }
            Mutation::VictoryPointMutation(amount) => self.victory_points += amount,
        }
        Ok(())
    }
}