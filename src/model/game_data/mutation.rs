use crate::model::card::CardId;
use crate::model::game_data::{GameData, MAX_OXYGEN, MAX_TEMPERATURE};
use crate::model::game_data::board::tile::Tile;
use crate::model::resource::Resource;
use crate::model::tag::Tag;

pub enum Mutation {
    Normal(NormalMutation),
    TilePlacing(Tile, (usize, usize)),
}

pub enum NormalMutation {
    CompositeMutation(Vec<NormalMutation>),
    CardPlayingMutation(CardId),
    ProductionMutation(Resource, i32),
    ResourceMutation(Resource, i32),
    TrMutation(i32),
    TemperatureIncreaseMutation,
    OxygenIncreaseMutation,
    VictoryPointMutation(i32),
    TileQueuingMutation(Tile),
    TagMutation(Tag),
    CardDrawMutation(i32),
    PassMutation,
}

const RESOURCES: [Resource; 6] = [
    Resource::MegaCredit,
    Resource::Steel,
    Resource::Titanium,
    Resource::Plant,
    Resource::Energy,
    Resource::Heat
];

impl GameData {
    pub fn mutate(&mut self, mutation: &Mutation) -> Result<(), ()> {
        match mutation {
            Mutation::TilePlacing(tile, position) => {
                if Some(tile) == self.tile_queue.last() {
                    self.tile_queue.pop();
                    self.board.place_tile(tile, *position)
                } else {
                    Err(())
                }
            }
            Mutation::Normal(normal_mutation) => {
                if !self.tile_queue.is_empty() {
                    return Err(());
                }
                let clone = self.clone();
                match self.apply_normal_mutation(normal_mutation) {
                    Ok(_) => Ok(()),
                    Err(_) => {
                        *self = clone;
                        Err(())
                    }
                }
            }
        }
    }

    fn apply_normal_mutation(&mut self, mutation: &NormalMutation) -> Result<(), ()> {
        match mutation {
            NormalMutation::CompositeMutation(mutations) => {
                for mutation in mutations {
                    self.apply_normal_mutation(mutation)?;
                }
            }
            NormalMutation::ProductionMutation(resource, amount) => {
                let production_value = self.productions
                    .get_mut(resource)
                    .expect("Production should be initialized in Game struct");
                *production_value += amount;
                if *production_value < Self::minimum_production_value_of(&resource) {
                    return Err(());
                }
            }
            NormalMutation::ResourceMutation(resource, amount) => {
                let resource_value = self.resources
                    .get_mut(resource)
                    .expect("Resource should be initialized in Game struct");
                *resource_value -= amount;
                if *resource_value < 0 { return Err(()); }
            }
            NormalMutation::TrMutation(amount) => self.tr += amount,
            NormalMutation::TemperatureIncreaseMutation => {
                if self.temperature < MAX_TEMPERATURE {
                    self.temperature += 2;
                    self.tr += 1;
                }
            }
            NormalMutation::OxygenIncreaseMutation => {
                if self.oxygen < MAX_OXYGEN {
                    self.oxygen += 1;
                    self.tr += 1;
                }
            }
            NormalMutation::VictoryPointMutation(amount) => self.victory_points += amount,
            NormalMutation::TileQueuingMutation(tile) => {
                self.tile_queue.push(tile.clone());
            }
            NormalMutation::TagMutation(tag) => {
                *self.tags.get_mut(tag).expect("Tag should be in the map") += 1;
            }
            NormalMutation::CardDrawMutation(amount) => {
                self.draw_cards(*amount);
            }
            NormalMutation::PassMutation => {
                self.generation += 1;

                *self.resources.get_mut(&Resource::MegaCredit).unwrap() += self.tr;
                for resource in RESOURCES {
                    *self.resources.get_mut(&resource).unwrap() +=
                        self.productions.get(&resource).unwrap();
                }

                self.draw_cards(4);
            }
            NormalMutation::CardPlayingMutation(_) => {}
        }
        Ok(())
    }

    fn minimum_production_value_of(resource: &Resource) -> i32 {
        match resource {
            Resource::MegaCredit => -5,
            _ => 0
        }
    }

    fn draw_cards(&mut self, count: i32) {
        for _ in 0..count {
            let card_id = self.cards_to_be_drawn.pop()
                .expect("The draw deck shouldn't run out");
            self.cards_in_hand.insert(card_id);
        }
    }
}
