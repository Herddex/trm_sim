use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;

use board::Board;
use crate::model::card::card_compendium::CARD_COMPENDIUM;

use crate::model::card::CardId;
use crate::model::game_data::board::tile::Tile;
use crate::model::resource::Resource;
use crate::model::tag::Tag;
use crate::model::game_data::board::game_map::GameMap;
use crate::model::game_data::game_memento::GameMemento;

pub(crate) mod mutation;
pub(crate) mod board;
pub(crate) mod requirement;
pub(crate) mod game_memento;

const INITIAL_TR: i32 = 14;
const INITIAL_TEMPERATURE: i32 = -30;
const INITIAL_OXYGEN: i32 = 0;
const INITIAL_MEGA_CREDITS: i32 = 42;
const INITIAL_PRODUCTION: i32 = 1;

const MAX_TEMPERATURE: i32 = 8;
const MAX_OXYGEN: i32 = 14;


#[derive(Clone)]
pub struct GameData {
    generation: u32,
    tr: i32,

    oxygen: i32,
    temperature: i32,

    resources: HashMap<Resource, i32>,
    productions: HashMap<Resource, i32>,

    board: Board,

    tags: HashMap<Tag, i32>,

    cards_in_hand: HashSet<CardId>,
    played_cards: HashSet<CardId>,
    cards_to_be_drawn: Vec<CardId>,

    victory_points: i32,

    tile_stack: Vec<Tile>,
}

impl GameData {
    pub fn new(map: &'static GameMap) -> GameData {
        let mut cards_to_be_drawn =
            Vec::from_iter(CARD_COMPENDIUM.keys().map(|key| *key));

        let mut rng = rand::thread_rng();
        cards_to_be_drawn.shuffle(&mut rng);

        GameData {
            generation: 1,
            tr: INITIAL_TR,
            oxygen: INITIAL_OXYGEN,
            temperature: INITIAL_TEMPERATURE,
            resources: HashMap::from([
                (Resource::MegaCredit, INITIAL_MEGA_CREDITS),
                (Resource::Steel, 0),
                (Resource::Titanium, 0),
                (Resource::Plant, 0),
                (Resource::Energy, 0),
                (Resource::Heat, 0)
            ]),
            productions: HashMap::from([
                (Resource::MegaCredit, INITIAL_PRODUCTION),
                (Resource::Steel, INITIAL_PRODUCTION),
                (Resource::Titanium, INITIAL_PRODUCTION),
                (Resource::Plant, INITIAL_PRODUCTION),
                (Resource::Energy, INITIAL_PRODUCTION),
                (Resource::Heat, INITIAL_PRODUCTION),
            ]),
            board: Board::new(map),
            tags: HashMap::from([
                (Tag::Builder, 0),
                (Tag::Space, 0),
                (Tag::Earth, 0),
                (Tag::Jovian, 0),
                (Tag::Power, 0),
                (Tag::Science, 0),
                (Tag::Plant, 0),
                (Tag::Microbe, 0)
            ]),

            cards_in_hand: (0..10).map(|_|
                cards_to_be_drawn.pop().expect("There should be at least 10 cards to draw"))
                .collect(),
            played_cards: HashSet::new(),
            cards_to_be_drawn,

            victory_points: INITIAL_TR,
            tile_stack: Vec::new(),
        }
    }

    pub fn save(&self) -> GameMemento {
        GameMemento::new(self.clone())
    }

    fn resource(&mut self, resource: &Resource) -> &mut i32 {
        self.resources.get_mut(resource)
            .expect("GameData.resources should have every resource as a key")
    }
}
