use lazy_static::lazy_static;

use StandardProject::*;

use crate::model::game_data::board::tile::Tile;
use crate::model::game_data::mutation::Mutation;
use crate::model::game_data::mutation::normal_mutation::NormalMutation;
use crate::model::game_data::mutation::normal_mutation::NormalMutation::{
    ProductionMutation, ResourceMutation, TemperatureIncreaseMutation, TileQueuingMutation};
use crate::model::resource::Resource::{Energy, MegaCredit};

// #[derive(PartialEq)]
pub enum StandardProject { City, Greenery, Aquifer, Asteroid, PowerPlant }

lazy_static! {
    static ref STANDARD_CITY: Mutation = Mutation::Normal(NormalMutation::CompositeMutation(vec![
        ResourceMutation(MegaCredit, 25),
        TileQueuingMutation(Tile::City),
        ProductionMutation(MegaCredit, 1)
    ]));
    static ref STANDARD_GREENERY: Mutation = Mutation::Normal(NormalMutation::CompositeMutation(vec![
        ResourceMutation(MegaCredit, 23),
        TileQueuingMutation(Tile::Greenery)
    ]));
    static ref STANDARD_AQUIFER: Mutation = Mutation::Normal(NormalMutation::CompositeMutation(vec![
        ResourceMutation(MegaCredit, 18),
        TileQueuingMutation(Tile::Ocean),
    ]));
    static ref STANDARD_ASTEROID: Mutation = Mutation::Normal(NormalMutation::CompositeMutation(vec![
        ResourceMutation(MegaCredit, 14),
        TemperatureIncreaseMutation
    ]));
    static ref STANDARD_POWER_PLANT: Mutation = Mutation::Normal(NormalMutation::CompositeMutation(vec![
        ResourceMutation(MegaCredit, 11),
        ProductionMutation(Energy, 1)
    ]));

}

impl StandardProject {
    pub fn mutation(&self) -> &Mutation {
        match self {
            City => &STANDARD_CITY,
            Greenery => &STANDARD_GREENERY,
            Aquifer => &STANDARD_AQUIFER,
            Asteroid => &STANDARD_ASTEROID,
            PowerPlant => &STANDARD_POWER_PLANT
        }
    }
}