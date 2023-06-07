use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use rand::seq::SliceRandom;

use board::Board;

use crate::model::card::card_compendium::CARD_COMPENDIUM;
use crate::model::card::CardId;
use crate::model::game::board::game_map::GameMap;
use crate::model::game::board::tile::Tile;
use crate::model::resource::Resource;
use crate::model::resource::Resource::{Energy, Heat, MegaCredit, Plant, Steel, Titanium};
use crate::model::tag::Tag;

pub(crate) mod board;
pub(crate) mod mutation;

const INITIAL_TR: i32 = 14;
const INITIAL_TEMPERATURE: i32 = -30;
const INITIAL_OXYGEN: i32 = 0;
const INITIAL_MEGA_CREDITS: i32 = 42;
const INITIAL_PRODUCTION: i32 = 1;

const MAX_TEMPERATURE: i32 = 8;
const MAX_OXYGEN: i32 = 14;
const MAX_OCEANS: i32 = 9;

const LAST_GENERATION: u32 = 14;

#[derive(Clone)]
pub struct Game {
    generation: u32,
    tr: i32,

    oxygen: i32,
    temperature: i32,
    oceans: i32,

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

impl Game {
    pub fn new(map: &'static GameMap) -> Game {
        let mut cards_to_be_drawn = Vec::from_iter(CARD_COMPENDIUM.keys().copied());

        let mut rng = rand::thread_rng();
        cards_to_be_drawn.shuffle(&mut rng);

        Game {
            generation: 1,
            tr: INITIAL_TR,
            oxygen: INITIAL_OXYGEN,
            temperature: INITIAL_TEMPERATURE,
            oceans: 0,
            resources: HashMap::from([
                (MegaCredit, INITIAL_MEGA_CREDITS),
                (Steel, 0),
                (Titanium, 0),
                (Plant, 0),
                (Energy, 0),
                (Heat, 0),
            ]),
            productions: HashMap::from([
                (MegaCredit, INITIAL_PRODUCTION),
                (Steel, INITIAL_PRODUCTION),
                (Titanium, INITIAL_PRODUCTION),
                (Plant, INITIAL_PRODUCTION),
                (Energy, INITIAL_PRODUCTION),
                (Heat, INITIAL_PRODUCTION),
            ]),
            board: Board::new(map),
            tags: HashMap::from([
                (Tag::Builder, 0),
                (Tag::Space, 0),
                (Tag::Earth, 0),
                (Tag::Jovian, 0),
                (Tag::Plant, 0),
                (Tag::Microbe, 0),
                (Tag::Power, 0),
                (Tag::Science, 0),
                (Tag::City, 0),
            ]),

            cards_in_hand: (0..10)
                .map(|_| {
                    cards_to_be_drawn
                        .pop()
                        .expect("There should be at least 10 cards to draw")
                })
                .collect(),
            played_cards: HashSet::new(),
            cards_to_be_drawn,

            victory_points: INITIAL_TR,
            tile_stack: Vec::new(),
        }
    }
    pub fn can_place(&self, tile: &Tile) -> bool {
        self.board.can_place(tile) && (*tile != Tile::Ocean || self.oceans < MAX_OCEANS)
    }
    pub fn is_over(&self) -> bool {
        self.generation > LAST_GENERATION
    }
    pub fn is_won(&self) -> bool {
        self.is_over()
            && self.oxygen == MAX_OXYGEN
            && self.temperature == MAX_TEMPERATURE
            && self.oceans == MAX_OCEANS
    }
    pub fn resource(&self, resource: &Resource) -> i32 {
        *self.resources.get(resource).unwrap()
    }
    pub fn production(&self, resource: &Resource) -> i32 {
        *self.productions.get(resource).unwrap()
    }
    pub fn tag(&self, tag: Tag) -> i32 {
        *self.tags.get(&tag).unwrap()
    }
    fn resource_mut(&mut self, resource: &Resource) -> &mut i32 {
        self.resources.get_mut(resource).unwrap()
    }
    pub fn oxygen(&self) -> i32 {
        self.oxygen
    }
    pub fn temperature(&self) -> i32 {
        self.temperature
    }
    pub fn oceans(&self) -> i32 {
        self.oceans
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CARDS:")?;
        for card_id in self.cards_in_hand.iter() {
            writeln!(f, "{}", CARD_COMPENDIUM.get(card_id).unwrap())?;
        }

        writeln!(f, "Generation: {}", max(LAST_GENERATION, self.generation))?;
        writeln!(
            f,
            "Terraforming Rating: {}; Victory Points: {}",
            self.tr, self.victory_points
        )?;
        writeln!(
            f,
            "Oxygen: {} / {}%; Temperature: {:+} / {:+} C; Oceans: {} / {}",
            self.oxygen, MAX_OXYGEN, self.temperature, MAX_TEMPERATURE, self.oceans, MAX_OCEANS
        )?;
        writeln!(f)?;

        writeln!(f, "Resources / production:")?;
        writeln!(
            f,
            "Megacredits:  {} / {}",
            self.resource(&MegaCredit),
            self.production(&MegaCredit)
        )?;
        writeln!(
            f,
            "Steel:        {} / {}",
            self.resource(&Steel),
            self.production(&Steel)
        )?;
        writeln!(
            f,
            "Titanium:     {} / {}",
            self.resource(&Titanium),
            self.production(&Titanium)
        )?;
        writeln!(
            f,
            "Plants:       {} / {}",
            self.resource(&Plant),
            self.production(&Plant)
        )?;
        writeln!(
            f,
            "Energy:       {} / {}",
            self.resource(&Energy),
            self.production(&Energy)
        )?;
        writeln!(
            f,
            "Heat:         {} / {}",
            self.resource(&Heat),
            self.production(&Heat)
        )?;

        writeln!(f)?;
        writeln!(f, "Tags: {:?}", self.tags)?;
        writeln!(f)?;

        writeln!(f, "{}", self.board)?;

        if self.is_over() {
            if self.is_won() {
                writeln!(f, "You won with {} victory points!", self.victory_points)?;
            } else {
                writeln!(f, "You lost.")?;
            }
        } else {
            write!(f, "NEXT ACTION: ")?;

            if self.tile_stack.is_empty() {
                writeln!(f, "Regular")?;
            } else {
                writeln!(f, "Place {:?}", self.tile_stack.last().unwrap())?;
            }
        }

        Ok(())
    }
}
