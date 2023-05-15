pub mod standard_project;

use lazy_static::lazy_static;
use crate::game::action::standard_project::StandardProject;
use crate::model::card::CardId;
use crate::model::game_data::GameData;
use crate::model::card::card_compendium::CARD_COMPENDIUM;
use crate::model::game_data::board::tile::Tile::Greenery;
use crate::model::game_data::mutation::Mutation;
use crate::model::game_data::mutation::normal_mutation::NormalMutation::{CompositeMutation, ResourceMutation, TemperatureIncreaseMutation, TileQueuingMutation};
use crate::model::resource::Resource::{Heat, Plant};

lazy_static!{
    static ref HEAT_CONVERSION_MUTATION: Mutation = Mutation::Normal(CompositeMutation(vec![
                    ResourceMutation(Heat, -8),
                    TemperatureIncreaseMutation
                ]));
    static ref PLANT_CONVERSION_MUTATION: Mutation = Mutation::Normal(CompositeMutation(vec![
                    ResourceMutation(Plant, -8),
                    TileQueuingMutation(Greenery)
                ]));
}

pub enum Action { Card(CardId), StandardProject(StandardProject), HeatConversion, PlantConversion }

impl Action {
    pub fn execute(self, game_data: &mut GameData) -> Result<(), ()> {
        match self {
            Self::Card(card_id) => {
                if let Some(card) = CARD_COMPENDIUM.get(&card_id) {
                    card.play(game_data)
                } else {
                    Err(())
                }
            }
            Action::StandardProject(standard_project) => {
                standard_project.mutation().apply(game_data)
            }
            Action::HeatConversion => {
                HEAT_CONVERSION_MUTATION.apply(game_data)
            }
            Action::PlantConversion => {
                PLANT_CONVERSION_MUTATION.apply(game_data)
            }
        }
    }
}