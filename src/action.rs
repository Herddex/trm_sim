use self::invalid_action::{ActionResult, InvalidActionError};
use crate::model::card::card_compendium::CARD_COMPENDIUM;
use crate::model::card::CardId;
use crate::model::game::board::tile::Tile;
use crate::model::game::mutation::Mutation;
use crate::model::game::Game;
use crate::model::resource::Resource;
use lazy_static::lazy_static;

pub mod invalid_action;

lazy_static! {
    static ref HEAT_CONVERSION: Mutation = Mutation::Composite(vec![
        Mutation::Resource(Resource::Heat, -8),
        Mutation::TemperatureIncrease(1)
    ]);
    static ref PLANT_CONVERSION: Mutation = Mutation::Composite(vec![
        Mutation::Resource(Resource::Plant, -8),
        Mutation::TileQueuing(Tile::Greenery)
    ]);
    static ref STANDARD_CITY: Mutation = Mutation::Composite(vec![
        Mutation::Resource(Resource::MegaCredit, -25),
        Mutation::TileQueuing(Tile::City),
        Mutation::Production(Resource::MegaCredit, 1)
    ]);
    static ref STANDARD_GREENERY: Mutation = Mutation::Composite(vec![
        Mutation::Resource(Resource::MegaCredit, -23),
        Mutation::TileQueuing(Tile::Greenery)
    ]);
    static ref STANDARD_AQUIFER: Mutation = Mutation::Composite(vec![
        Mutation::Resource(Resource::MegaCredit, -18),
        Mutation::TileQueuing(Tile::Ocean),
    ]);
    static ref STANDARD_ASTEROID: Mutation = Mutation::Composite(vec![
        Mutation::Resource(Resource::MegaCredit, -14),
        Mutation::TemperatureIncrease(1)
    ]);
    static ref STANDARD_POWER_PLANT: Mutation = Mutation::Composite(vec![
        Mutation::Resource(Resource::MegaCredit, -11),
        Mutation::Production(Resource::Energy, 1)
    ]);
}

#[derive(Debug)]
pub enum Action {
    Card(CardId),
    StandardPowerPlant,
    StandardAsteroid,
    StandardAquifer,
    StandardGreenery,
    StandardCity,
    HeatConversion,
    PlantConversion,
    TilePlacement((usize, usize)),
    Pass,
}

impl Action {
    pub fn execute(&self, game: &mut Game) -> ActionResult {
        match self {
            Self::Card(card_id) => {
                if let Some(card) = CARD_COMPENDIUM.get(card_id) {
                    card.play(game)
                } else {
                    InvalidActionError::new(format!("Card #{:0>3} does not exist", *card_id))
                        .into_err()
                }
            }
            Self::StandardPowerPlant => STANDARD_POWER_PLANT.apply(game),
            Self::StandardAsteroid => STANDARD_ASTEROID.apply(game),
            Self::StandardAquifer => STANDARD_AQUIFER.apply(game),
            Self::StandardGreenery => STANDARD_GREENERY.apply(game),
            Self::StandardCity => STANDARD_CITY.apply(game),
            Self::HeatConversion => HEAT_CONVERSION.apply(game),
            Self::PlantConversion => PLANT_CONVERSION.apply(game),
            Self::TilePlacement(position) => Mutation::TilePlacement(*position).apply(game),
            Self::Pass => Mutation::Pass.apply(game),
        }
    }
}
