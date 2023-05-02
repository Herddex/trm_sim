use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use crate::model::card::{Card, CardId};
use crate::model::card::card_builder::CardBuilder;
use crate::model::game_data::board::tile::Tile;
use crate::model::game_data::board::tile::Tile::{Greenery, Ocean};
use crate::model::game_data::mutation::NormalMutation::{CardDrawMutation, OxygenIncreaseMutation, ProductionMutation, ResourceMutation, TemperatureIncreaseMutation, TileQueuingMutation, TrMutation};
use crate::model::game_data::requirement::NewRequirement;
use crate::model::resource::Resource::{Energy, Heat, MegaCredit, Plant, Steel, Titanium};
use crate::model::tag::Tag;
use crate::model::tag::Tag::{Builder, City, Earth, Jovian, Microbe, Power, Science, Space};

lazy_static! {
    static ref CARD_COMPENDIUM: HashMap<CardId, Arc<Card>> = build_card_compendium();
}

pub fn build_card_compendium() -> HashMap<CardId, Arc<Card>> {
    HashMap::from([
        (1, CardBuilder::new()
            .cost(8)
            .tags(vec![Jovian, Builder])
            .requirement(NewRequirement::MaxOxygen(5))
            .victory_points(2)
            .build()),
        (3, CardBuilder::new()
            .cost(13)
            .tags(vec![Power, Builder])
            .mutation(ProductionMutation(Energy, 1))
            .mutation(TemperatureIncreaseMutation)
            .build()),
        (4, CardBuilder::new()
            .cost(13)
            .requirement(NewRequirement::MinOcean(3))
            .mutation(ProductionMutation(MegaCredit, -1))
            .mutation(ProductionMutation(Plant, 2))
            .build()),
        (9, space_event()
            .cost(14)
            .mutation(TemperatureIncreaseMutation)
            .mutation(ResourceMutation(Titanium, 2))
            .build()),
        (10, space_event()
            .cost(21)
            .mutation(TemperatureIncreaseMutation)
            .mutation(TileQueuingMutation(Ocean))
            .build()),
        (11, space_event()
            .cost(27)
            .mutation(TemperatureIncreaseMutation)
            .mutation(TemperatureIncreaseMutation)
            .mutation(ResourceMutation(Titanium, 4))
            .build()),
        (16, martian_city()
            .cost(24)
            .requirement(NewRequirement::MaxOxygen(7))
            .mutation(ProductionMutation(MegaCredit, 3))
            .mutation(ResourceMutation(Plant, 3))
            .victory_points(1)
            .build()),
        (17, martian_city()
            .cost(18)
            .mutation(ProductionMutation(MegaCredit, 3))
            .build()),
        (18, CardBuilder::new()
            .cost(28)
            .tags(vec![Jovian, Space])
            .requirement(NewRequirement::MinOxygen(2))
            .mutation(ProductionMutation(Heat, 2))
            .mutation(ProductionMutation(Plant, 2))
            .victory_points(2)
            .build()),
        (19, space_event()
            .tags(vec![Earth])
            .mutation(ResourceMutation(Plant, 3))
            .mutation(TileQueuingMutation(Ocean))
            .build()),
        (22, CardBuilder::new()
            .cost(15)
            .mutation(ProductionMutation(MegaCredit, -2))
            .mutation(ProductionMutation(Heat, 3))
            .mutation(TileQueuingMutation(Ocean))
            .build()),
        (26, CardBuilder::new()
            .cost(16)
            .tags(vec![Tag::Plant, Builder])
            .requirement(NewRequirement::MinTemperature(-12))
            .mutation(ResourceMutation(Plant, 3))
            .mutation(ProductionMutation(MegaCredit, 2))
            .victory_points(1)
            .build()),
        (29, martian_city()
            .cost(16)
            .requirement(NewRequirement::MaxOxygen(9))
            .mutation(ProductionMutation(MegaCredit, 3))
            .build()),
        (30, CardBuilder::new()
            .cost(13)
            .tags(vec![Earth, Power])
            .mutation(ProductionMutation(MegaCredit, -2))
            .mutation(ProductionMutation(Heat, 2))
            .mutation(ProductionMutation(Energy, 2))
            .build()),
        (32, martian_city()
            .cost(18)
            .mutation(ProductionMutation(Energy, -1))
            .mutation(ProductionMutation(Steel, 2))
            .build()),
        (36, CardBuilder::new()
            .cost(14)
            .event()
            .mutation(TrMutation(2))
            .build()),
        (37, space_event()
            .cost(31)
            .mutation(ProductionMutation(Plant, 1))
            .mutation(TrMutation(2))
            .mutation(TemperatureIncreaseMutation)
            .build()),
        (39, space_event()
            .cost(31)
            .mutation(TemperatureIncreaseMutation)
            .mutation(TemperatureIncreaseMutation)
            .mutation(TemperatureIncreaseMutation)
            .mutation(ResourceMutation(Steel, 4))
            .build()),
        (40, CardBuilder::new()
            .cost(30)
            .tags(vec![Jovian, Space])
            .mutation(ProductionMutation(Titanium, 2))
            .victory_points(2)
            .build()),
        (41, CardBuilder::new()
            .cost(12)
            .tags(vec![Builder])
            .mutation(ProductionMutation(Plant, -1))
            .mutation(ProductionMutation(MegaCredit, 4))
            .victory_points(1)
            .build()),
        (42, CardBuilder::new()
            .cost(6)
            .tags(vec![Microbe])
            .requirement(NewRequirement::MaxTemperature(-18))
            .mutation(ProductionMutation(Plant, 1))
            .build()),
        (43, CardBuilder::new()
            .cost(6)
            .tags(vec![Builder])
            .mutation(ProductionMutation(Energy, -1))
            .mutation(ProductionMutation(Heat, 3))
            .build()),
        (45, CardBuilder::new()
            .cost(10)
            .tags(vec![Power, Builder])
            .mutation(ProductionMutation(MegaCredit, -2))
            .mutation(ProductionMutation(Energy, 3))
            .build()),
        (47, CardBuilder::new()
            .cost(10)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::MinOcean(5))
            .mutation(ProductionMutation(Plant, 2))
            .mutation(ResourceMutation(Plant, 1))
            .build()),
        (48, CardBuilder::new()
            .cost(9)
            .tags(vec![Tag::Plant])
            .mutation(ProductionMutation(Plant, 1))
            .build()),
        (53, CardBuilder::new()
            .cost(18)
            .requirement(NewRequirement::MinTemperature(0))
            .mutation(TileQueuingMutation(Ocean))
            .mutation(TileQueuingMutation(Ocean))
            .victory_points(2)
            .build()),
        (55, CardBuilder::new()
            .cost(17)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::MinOcean(6))
            .mutation(ProductionMutation(MegaCredit, 2))
            .mutation(ProductionMutation(Plant, 3))
            .mutation(ResourceMutation(Plant, 2))
            .victory_points(1)
            .build()),
        (58, CardBuilder::new()
            .cost(32)
            .tags(vec![Jovian, Space, Power])
            .requirement(NewRequirement::Tag(Jovian, 1))
            .mutation(ProductionMutation(Heat, 3))
            .mutation(ProductionMutation(Energy, 3))
            .victory_points(1)
            .build()),
        (60, CardBuilder::new()
            .cost(13)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::MinTemperature(-4))
            .mutation(ProductionMutation(Plant, 3))
            .mutation(ResourceMutation(Plant, 1))
            .victory_points(1)
            .build()),
        (63, CardBuilder::new()
            .cost(12)
            .event()
            .mutation(OxygenIncreaseMutation)
            .mutation(ResourceMutation(Steel, 2))
            .build()),
        (75, space_event()
            .cost(23)
            .mutation(OxygenIncreaseMutation)
            .mutation(TileQueuingMutation(Ocean))
            .mutation(ResourceMutation(Plant, 2))
            .build()),
        (77, CardBuilder::new()
            .cost(11)
            .tags(vec![Science, Space, Power])
            .mutation(ProductionMutation(Energy, 1))
            .mutation(ResourceMutation(Titanium, 2))
            .build()),
        (78, space_event()
            .cost(23)
            .mutation(TileQueuingMutation(Ocean))
            .mutation(TileQueuingMutation(Ocean))
            .build()),
        (80, space_event()
            .cost(36)
            .mutation(TileQueuingMutation(Ocean))
            .mutation(TileQueuingMutation(Ocean))
            .mutation(TemperatureIncreaseMutation)
            .mutation(TemperatureIncreaseMutation)
            .build()),
        (83, CardBuilder::new()
            .cost(17)
            .tags(vec![Power, Space])
            .mutation(ProductionMutation(Energy, 3))
            .build()),
        (87, CardBuilder::new()
            .cost(11)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::MinTemperature(-16))
            .mutation(ProductionMutation(Plant, 1))
            .mutation(ResourceMutation(Plant, 3))
            .build()),
        (88, CardBuilder::new()
            .cost(6)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::MinTemperature(-14))
            .mutation(ProductionMutation(Plant, 1))
            .mutation(ResourceMutation(Plant, 1))
            .build()),
        (89, CardBuilder::new()
            .cost(7)
            .tags(vec![Power, Builder])
            .mutation(ProductionMutation(MegaCredit, -1))
            .mutation(ProductionMutation(Energy, 2))
            .build()),
        (93, CardBuilder::new()
            .cost(10)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::MinTemperature(-10))
            .mutation(ProductionMutation(Plant, 2))
            .mutation(ResourceMutation(Plant, 2))
            .build()),
        (100, CardBuilder::new()
            .cost(1)
            .tags(vec![Power, Builder])
            .mutation(ProductionMutation(MegaCredit, -1))
            .mutation(ProductionMutation(Energy, 1))
            .build()),
        (108, martian_city()
            .requirement(NewRequirement::MinOxygen(12))
            .mutation(ProductionMutation(MegaCredit, 4))
            .mutation(ResourceMutation(Plant, 2))
            .victory_points(1)
            .build()),
        (113, CardBuilder::new()
            .cost(11)
            .tags(vec![Power, Builder])
            .mutation(ProductionMutation(Energy, 1))
            .victory_points(1)
            .build()),
        (114, CardBuilder::new()
            .cost(11)
            .tags(vec![Science])
            .requirement(NewRequirement::MinOxygen(7))
            .victory_points(2)
            .build()),
        (117, CardBuilder::new()
            .cost(11)
            .tags(vec![Power, Builder])
            .mutation(ProductionMutation(Energy, 2))
            .build()),
        (118, CardBuilder::new()
            .cost(16)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::MinTemperature(4))
            .mutation(ProductionMutation(MegaCredit, 2))
            .mutation(ProductionMutation(Plant, 2))
            .mutation(ResourceMutation(Plant, 2))
            .victory_points(2)
            .build()),
        (119, CardBuilder::new()
            .cost(2)
            .requirement(NewRequirement::MaxOcean(3))
            .victory_points(1)
            .build()),
        (122, CardBuilder::new()
            .cost(4)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::MinOcean(3))
            .mutation(ProductionMutation(Plant, 1))
            .mutation(ResourceMutation(Plant, -1))
            .build()),
        (126, CardBuilder::new()
            .cost(11)
            .tags(vec![Builder])
            .mutation(ProductionMutation(Energy, -1))
            .mutation(ProductionMutation(Heat, 4))
            .build()),
        (127, CardBuilder::new()
            .cost(11)
            .event()
            .mutation(TileQueuingMutation(Ocean))
            .build()),
        (132, CardBuilder::new()
            .cost(14)
            .tags(vec![Science, Power, Builder])
            .requirement(NewRequirement::Tag(Power, 2))
            .mutation(ProductionMutation(Energy, 3))
            .build()),
        (136, CardBuilder::new()
            .cost(4)
            .tags(vec![Power, Builder])
            .mutation(ProductionMutation(Energy, 1))
            .build()),
        (143, space_event()
            .tags(vec![Earth])
            .mutation(TileQueuingMutation(Ocean))
            .mutation(CardDrawMutation(2))
            .mutation(ResourceMutation(Plant, 5))
            .victory_points(2)
            .build()),
        (145, CardBuilder::new()
            .cost(18)
            .tags(vec![Power, Builder])
            .requirement(NewRequirement::Tag(Science, 2))
            .mutation(ProductionMutation(Energy, 3))
            .victory_points(1)
            .build()),
        (146, CardBuilder::new()
            .cost(8)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::MinOcean(3))
            .mutation(ProductionMutation(Plant, 2))
            .mutation(ResourceMutation(Plant, -2))
            .build()),
        (155, CardBuilder::new()
            .cost(16)
            .tags(vec![Science, Microbe])
            .requirement(NewRequirement::MaxTemperature(-14))
            .mutation(ProductionMutation(Plant, 2))
            .build()),
        (158, CardBuilder::new()
            .cost(12)
            .tags(vec![Microbe, Builder])
            .mutation(ProductionMutation(Energy, 1))
            .mutation(ProductionMutation(Steel, 1))
            .build()),
        (159, CardBuilder::new()
            .cost(7)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::MinTemperature(-24))
            .mutation(ResourceMutation(Plant, 1))
            .build()),
        (161, space_event()
            .cost(15)
            .mutation(TileQueuingMutation(Ocean))
            .mutation(CardDrawMutation(1))
            .build()),
        (162, space_event()
            .cost(7)
            .tags(vec![Earth])
            .mutation(ProductionMutation(Heat, 2))
            .build()),
        (168, CardBuilder::new()
            .cost(6)
            .tags(vec![Power, Builder])
            .requirement(NewRequirement::MinOxygen(7))
            .mutation(ProductionMutation(Energy, 1))
            .victory_points(1)
            .build()),
        (169, CardBuilder::new()
            .cost(16)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::MinTemperature(-6))
            .mutation(ProductionMutation(Plant, 1))
            .mutation(ProductionMutation(MegaCredit, 2))
            .mutation(ResourceMutation(Plant, 1))
            .victory_points(2)
            .build()),
        (170, space_event()
            .cost(26)
            .mutation(ProductionMutation(Heat, 3))
            .mutation(ProductionMutation(Plant, 1))
            .build()),
        (171, CardBuilder::new()
            .cost(5)
            .tags(vec![Builder])
            .mutation(ProductionMutation(Energy, -2))
            .mutation(ProductionMutation(Plant, 1))
            .mutation(TrMutation(1))
            .build()),
        (176, CardBuilder::new()
            .cost(10)
            .tags(vec![Tag::Plant, Builder])
            .requirement(NewRequirement::MinTemperature(-20))
            .mutation(ProductionMutation(MegaCredit, 1))
            .mutation(ResourceMutation(Plant, 2))
            .victory_points(1)
            .build()),
        (179, CardBuilder::new()
            .cost(9)
            .tags(vec![Builder])
            .mutation(ProductionMutation(Energy, -1))
            .mutation(ProductionMutation(Plant, 1))
            .build()),
        (180, CardBuilder::new()
            .cost(6)
            .tags(vec![Builder])
            .mutation(ProductionMutation(Energy, -1))
            .mutation(ProductionMutation(Titanium, 1))
            .mutation(ProductionMutation(MegaCredit, 1))
            .build()),
        (181, CardBuilder::new()
            .cost(5)
            .event()
            .requirement(NewRequirement::MinTemperature(2))
            .mutation(TileQueuingMutation(Ocean))
            .build()),
        (182, martian_city()
            .mutation(ProductionMutation(MegaCredit, 3))
            .victory_points(-2)
            .build()),
        (190, CardBuilder::new()
            .cost(1)
            .event()
            .mutation(ResourceMutation(Heat, -5))
            .mutation(ResourceMutation(Plant, 4))
            .build()),
        (191, CardBuilder::new()
            .cost(8)
            .event()
            .requirement(NewRequirement::MinTemperature(-8))
            .mutation(TileQueuingMutation(Ocean))
            .build()),
        (193, CardBuilder::new()
            .cost(15)
            .tags(vec![Tag::Plant])
            .requirement(NewRequirement::Tag(Science, 2))
            .mutation(TileQueuingMutation(Greenery))
            .build()),
        (203, CardBuilder::new()
            .cost(35)
            .tags(vec![Space])
            .mutation(ProductionMutation(Heat, 7))
            .build()),
        (205, CardBuilder::new()
            .cost(8)
            .tags(vec![Builder])
            .mutation(ProductionMutation(Energy, -1))
            .mutation(TrMutation(2))
            .build()),
    ]).into_iter()
        .map(|(k, v)| (k as usize, Arc::new(v)))
        .collect()
}

fn space_event() -> CardBuilder {
    CardBuilder::new()
        .tags(vec![Space])
        .event()
}

fn martian_city() -> CardBuilder {
    CardBuilder::new()
        .tags(vec![City, Builder])
        .mutation(ResourceMutation(Energy, -1))
        .mutation(TileQueuingMutation(Tile::City))
}