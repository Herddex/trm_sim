use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::model::card::card_builder::CardBuilder;
use crate::model::card::requirement::Requirement;
use crate::model::card::{Card, CardId};
use crate::model::game::board::tile::Tile;
use crate::model::game::board::tile::Tile::{Greenery, Ocean};
use crate::model::game::mutation::Mutation::*;
use crate::model::resource::Resource::*;
use crate::model::tag::Tag;
use crate::model::tag::Tag::{Builder, City, Earth, Jovian, Microbe, Power, Science, Space};

lazy_static! {
    pub(crate) static ref CARD_COMPENDIUM: HashMap<CardId, Card> = build_card_compendium();
    pub static ref ALL_CARD_IDS_IN_ASCENDING_ORDER: Vec<CardId> = build_card_id_list();
}

fn build_card_compendium() -> HashMap<CardId, Card> {
    [
        (
            1,
            CardBuilder::new()
                .cost(8)
                .tags(vec![Jovian, Builder])
                .requirement(Requirement::MaxOxygen(5))
                .victory_points(2),
        ),
        (
            3,
            CardBuilder::new()
                .cost(13)
                .tags(vec![Power, Builder])
                .mutation(Production(Energy, 1))
                .mutation(TemperatureIncrease(1)),
        ),
        (
            4,
            CardBuilder::new()
                .cost(13)
                .requirement(Requirement::MinOceanCount(3))
                .mutation(Production(MegaCredit, -1))
                .mutation(Production(Plant, 2)),
        ),
        (
            9,
            space_event()
                .cost(14)
                .mutation(TemperatureIncrease(1))
                .mutation(Resource(Titanium, 2)),
        ),
        (
            10,
            space_event()
                .cost(21)
                .mutation(TemperatureIncrease(1))
                .mutation(TilePlacement(Ocean)),
        ),
        (
            11,
            space_event()
                .cost(27)
                .mutation(TemperatureIncrease(2))
                .mutation(Resource(Titanium, 4)),
        ),
        (
            16,
            martian_city()
                .cost(24)
                .requirement(Requirement::MaxOxygen(7))
                .mutation(Production(MegaCredit, 3))
                .mutation(Resource(Plant, 3))
                .victory_points(1),
        ),
        (
            17,
            martian_city().cost(18).mutation(Production(MegaCredit, 3)),
        ),
        (
            18,
            CardBuilder::new()
                .cost(28)
                .tags(vec![Jovian, Space])
                .requirement(Requirement::MinOxygen(2))
                .mutation(Production(Heat, 2))
                .mutation(Production(Plant, 2))
                .victory_points(2),
        ),
        (
            19,
            space_event()
                .cost(16)
                .tags(vec![Earth])
                .mutation(Resource(Plant, 3))
                .mutation(TilePlacement(Ocean)),
        ),
        (
            22,
            CardBuilder::new()
                .cost(15)
                .mutation(Production(MegaCredit, -2))
                .mutation(Production(Heat, 3))
                .mutation(TilePlacement(Ocean)),
        ),
        (
            26,
            CardBuilder::new()
                .cost(16)
                .tags(vec![Tag::Plant, Builder])
                .requirement(Requirement::MinTemperature(-12))
                .mutation(Resource(Plant, 3))
                .mutation(Production(MegaCredit, 2))
                .victory_points(1),
        ),
        (
            29,
            martian_city()
                .cost(16)
                .requirement(Requirement::MaxOxygen(9))
                .mutation(Production(MegaCredit, 3)),
        ),
        (
            30,
            CardBuilder::new()
                .cost(13)
                .tags(vec![Earth, Power])
                .mutation(Production(MegaCredit, -2))
                .mutation(Production(Heat, 2))
                .mutation(Production(Energy, 2)),
        ),
        (
            32,
            martian_city()
                .cost(18)
                .mutation(Production(Energy, -1))
                .mutation(Production(Steel, 2)),
        ),
        (36, CardBuilder::new().cost(14).event().mutation(TR(2))),
        (
            37,
            space_event()
                .cost(31)
                .mutation(Production(Plant, 1))
                .mutation(TR(2))
                .mutation(TemperatureIncrease(1)),
        ),
        (
            39,
            space_event()
                .cost(31)
                .mutation(TemperatureIncrease(3))
                .mutation(Resource(Steel, 4)),
        ),
        (
            40,
            CardBuilder::new()
                .cost(30)
                .tags(vec![Jovian, Space])
                .mutation(Production(Titanium, 2))
                .victory_points(2),
        ),
        (
            41,
            CardBuilder::new()
                .cost(12)
                .tags(vec![Builder])
                .mutation(Production(Plant, -1))
                .mutation(Production(MegaCredit, 4))
                .victory_points(1),
        ),
        (
            42,
            CardBuilder::new()
                .cost(6)
                .tags(vec![Microbe])
                .requirement(Requirement::MaxTemperature(-18))
                .mutation(Production(Plant, 1)),
        ),
        (
            43,
            CardBuilder::new()
                .cost(6)
                .tags(vec![Builder])
                .mutation(Production(Energy, -1))
                .mutation(Production(Heat, 3)),
        ),
        (
            45,
            CardBuilder::new()
                .cost(10)
                .tags(vec![Power, Builder])
                .mutation(Production(MegaCredit, -2))
                .mutation(Production(Energy, 3)),
        ),
        (
            47,
            CardBuilder::new()
                .cost(10)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::MinOceanCount(5))
                .mutation(Production(Plant, 2))
                .mutation(Resource(Plant, 1)),
        ),
        (
            48,
            CardBuilder::new()
                .cost(9)
                .tags(vec![Tag::Plant])
                .mutation(Production(Plant, 1)),
        ),
        (
            53,
            CardBuilder::new()
                .cost(18)
                .requirement(Requirement::MinTemperature(0))
                .mutation(TilePlacement(Ocean))
                .mutation(TilePlacement(Ocean))
                .victory_points(2),
        ),
        (
            55,
            CardBuilder::new()
                .cost(17)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::MinOceanCount(6))
                .mutation(Production(MegaCredit, 2))
                .mutation(Production(Plant, 3))
                .mutation(Resource(Plant, 2))
                .victory_points(1),
        ),
        (
            58,
            CardBuilder::new()
                .cost(32)
                .tags(vec![Jovian, Space, Power])
                .requirement(Requirement::Tag(Jovian, 1))
                .mutation(Production(Heat, 3))
                .mutation(Production(Energy, 3))
                .victory_points(1),
        ),
        (
            60,
            CardBuilder::new()
                .cost(13)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::MinTemperature(-4))
                .mutation(Production(Plant, 3))
                .mutation(Resource(Plant, 1))
                .victory_points(1),
        ),
        (
            63,
            CardBuilder::new()
                .cost(12)
                .event()
                .mutation(OxygenIncrease(1))
                .mutation(Resource(Steel, 2)),
        ),
        (
            75,
            space_event()
                .cost(23)
                .mutation(OxygenIncrease(1))
                .mutation(TilePlacement(Ocean))
                .mutation(Resource(Plant, 2)),
        ),
        (
            77,
            CardBuilder::new()
                .cost(11)
                .tags(vec![Science, Space, Power])
                .mutation(Production(Energy, 1))
                .mutation(Resource(Titanium, 2)),
        ),
        (
            78,
            space_event()
                .cost(23)
                .mutation(TilePlacement(Ocean))
                .mutation(TilePlacement(Ocean)),
        ),
        (
            80,
            space_event()
                .cost(36)
                .mutation(TilePlacement(Ocean))
                .mutation(TilePlacement(Ocean))
                .mutation(TemperatureIncrease(2)),
        ),
        (
            83,
            CardBuilder::new()
                .cost(17)
                .tags(vec![Power, Space])
                .mutation(Production(Energy, 3)),
        ),
        (
            87,
            CardBuilder::new()
                .cost(11)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::MinTemperature(-16))
                .mutation(Production(Plant, 1))
                .mutation(Resource(Plant, 3)),
        ),
        (
            88,
            CardBuilder::new()
                .cost(6)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::MinTemperature(-14))
                .mutation(Production(Plant, 1))
                .mutation(Resource(Plant, 1)),
        ),
        (
            89,
            CardBuilder::new()
                .cost(7)
                .tags(vec![Power, Builder])
                .mutation(Production(MegaCredit, -1))
                .mutation(Production(Energy, 2)),
        ),
        (
            93,
            CardBuilder::new()
                .cost(10)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::MinTemperature(-10))
                .mutation(Production(Plant, 2))
                .mutation(Resource(Plant, 2)),
        ),
        (
            100,
            CardBuilder::new()
                .cost(1)
                .tags(vec![Power, Builder])
                .mutation(Production(MegaCredit, -1))
                .mutation(Production(Energy, 1)),
        ),
        (
            108,
            martian_city()
                .cost(23)
                .requirement(Requirement::MinOxygen(12))
                .mutation(Production(MegaCredit, 4))
                .mutation(Resource(Plant, 2))
                .victory_points(1),
        ),
        (
            113,
            CardBuilder::new()
                .cost(11)
                .tags(vec![Power, Builder])
                .mutation(Production(Energy, 1))
                .victory_points(1),
        ),
        (
            114,
            CardBuilder::new()
                .cost(11)
                .tags(vec![Science])
                .requirement(Requirement::MinOxygen(7))
                .victory_points(2),
        ),
        (
            117,
            CardBuilder::new()
                .cost(11)
                .tags(vec![Power, Builder])
                .mutation(Production(Energy, 2)),
        ),
        (
            118,
            CardBuilder::new()
                .cost(16)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::MinTemperature(4))
                .mutation(Production(MegaCredit, 2))
                .mutation(Production(Plant, 2))
                .mutation(Resource(Plant, 2))
                .victory_points(2),
        ),
        (
            119,
            CardBuilder::new()
                .cost(2)
                .requirement(Requirement::MaxOceanCount(3))
                .victory_points(1),
        ),
        (
            122,
            CardBuilder::new()
                .cost(4)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::MinOceanCount(3))
                .mutation(Production(Plant, 1))
                .mutation(Resource(Plant, -1)),
        ),
        (
            126,
            CardBuilder::new()
                .cost(11)
                .tags(vec![Builder])
                .mutation(Production(Energy, -1))
                .mutation(Production(Heat, 4)),
        ),
        (
            127,
            CardBuilder::new()
                .cost(11)
                .event()
                .mutation(TilePlacement(Ocean)),
        ),
        (
            132,
            CardBuilder::new()
                .cost(14)
                .tags(vec![Science, Power, Builder])
                .requirement(Requirement::Tag(Power, 2))
                .mutation(Production(Energy, 3)),
        ),
        (
            136,
            CardBuilder::new()
                .cost(4)
                .tags(vec![Power, Builder])
                .mutation(Production(Energy, 1)),
        ),
        (
            143,
            space_event()
                .cost(36)
                .tags(vec![Earth])
                .mutation(TilePlacement(Ocean))
                .mutation(CardDraw(2))
                .mutation(Resource(Plant, 5))
                .victory_points(2),
        ),
        (
            145,
            CardBuilder::new()
                .cost(18)
                .tags(vec![Power, Builder])
                .requirement(Requirement::Tag(Science, 2))
                .mutation(Production(Energy, 3))
                .victory_points(1),
        ),
        (
            146,
            CardBuilder::new()
                .cost(8)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::MinOceanCount(3))
                .mutation(Production(Plant, 2))
                .mutation(Resource(Plant, -2)),
        ),
        (
            155,
            CardBuilder::new()
                .cost(16)
                .tags(vec![Science, Microbe])
                .requirement(Requirement::MaxTemperature(-14))
                .mutation(Production(Plant, 2)),
        ),
        (
            158,
            CardBuilder::new()
                .cost(12)
                .tags(vec![Microbe, Builder])
                .mutation(Production(Energy, 1))
                .mutation(Production(Steel, 1)),
        ),
        (
            159,
            CardBuilder::new()
                .cost(7)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::MinTemperature(-24))
                .mutation(Resource(Plant, 1)),
        ),
        (
            161,
            space_event()
                .cost(15)
                .mutation(TilePlacement(Ocean))
                .mutation(CardDraw(1)),
        ),
        (
            162,
            space_event()
                .cost(7)
                .tags(vec![Earth])
                .mutation(Production(Heat, 1))
                .mutation(Resource(Heat, 3)),
        ),
        (
            168,
            CardBuilder::new()
                .cost(6)
                .tags(vec![Power, Builder])
                .requirement(Requirement::MinOxygen(7))
                .mutation(Production(Energy, 1))
                .victory_points(1),
        ),
        (
            169,
            CardBuilder::new()
                .cost(16)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::MinTemperature(-6))
                .mutation(Production(Plant, 1))
                .mutation(Production(MegaCredit, 2))
                .mutation(Resource(Plant, 1))
                .victory_points(2),
        ),
        (
            170,
            space_event()
                .cost(26)
                .mutation(Production(Heat, 3))
                .mutation(Production(Plant, 1)),
        ),
        (
            171,
            CardBuilder::new()
                .cost(5)
                .tags(vec![Builder])
                .mutation(Production(Energy, -2))
                .mutation(Production(Plant, 1))
                .mutation(TR(1)),
        ),
        (
            176,
            CardBuilder::new()
                .cost(10)
                .tags(vec![Tag::Plant, Builder])
                .requirement(Requirement::MinTemperature(-20))
                .mutation(Production(MegaCredit, 1))
                .mutation(Resource(Plant, 2))
                .victory_points(1),
        ),
        (
            179,
            CardBuilder::new()
                .cost(9)
                .tags(vec![Builder])
                .mutation(Production(Energy, -1))
                .mutation(Production(Plant, 1)),
        ),
        (
            180,
            CardBuilder::new()
                .cost(6)
                .tags(vec![Builder])
                .mutation(Production(Energy, -1))
                .mutation(Production(Titanium, 1))
                .mutation(Production(MegaCredit, 1)),
        ),
        (
            181,
            CardBuilder::new()
                .cost(5)
                .event()
                .requirement(Requirement::MinTemperature(2))
                .mutation(TilePlacement(Ocean)),
        ),
        (
            190,
            CardBuilder::new()
                .cost(1)
                .event()
                .mutation(Resource(Heat, -5))
                .mutation(Resource(Plant, 4)),
        ),
        (
            191,
            CardBuilder::new()
                .cost(8)
                .event()
                .requirement(Requirement::MinTemperature(-8))
                .mutation(TilePlacement(Ocean)),
        ),
        (
            193,
            CardBuilder::new()
                .cost(15)
                .tags(vec![Tag::Plant])
                .requirement(Requirement::Tag(Science, 2))
                .mutation(TilePlacement(Greenery)),
        ),
        (
            203,
            CardBuilder::new()
                .cost(35)
                .tags(vec![Space])
                .mutation(Production(Heat, 7)),
        ),
        (
            205,
            CardBuilder::new()
                .cost(8)
                .tags(vec![Builder])
                .mutation(Production(Energy, -1))
                .mutation(TR(2)),
        ),
    ]
    .into_iter()
    .map(|(id, card_builder)| (id, card_builder.id(id).build()))
    .collect()
}

fn space_event() -> CardBuilder {
    CardBuilder::new().tags(vec![Space]).event()
}

fn martian_city() -> CardBuilder {
    CardBuilder::new()
        .tags(vec![City, Builder])
        .mutation(Production(Energy, -1))
        .mutation(TilePlacement(Tile::City))
}

fn build_card_id_list() -> Vec<CardId> {
    let mut card_ids = CARD_COMPENDIUM.keys().copied().collect::<Vec<CardId>>();

    card_ids.sort();

    card_ids
}
