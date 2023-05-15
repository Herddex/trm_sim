use std::cmp::min;

#[cfg(test)]
mod normal_mutation_test;

use crate::model::card::CardId;
use crate::model::game_data::{GameData, MAX_OXYGEN, MAX_TEMPERATURE};
use crate::model::game_data::board::tile::Tile;
use crate::model::resource::Resource;
use crate::model::resource::Resource::{Energy, Heat, MegaCredit, Plant, Steel, Titanium};
use crate::model::tag::Tag;

pub enum NormalMutation {
    CompositeMutation(Vec<NormalMutation>),
    ProductionMutation(Resource, i32),
    ResourceMutation(Resource, i32),
    BuilderCardPaymentMutation(i32),
    SpaceCardPaymentMutation(i32),
    TrMutation(i32),
    TemperatureIncreaseMutation,
    OxygenIncreaseMutation,
    VictoryPointMutation(i32),
    TileQueuingMutation(Tile),
    TagMutation(Tag),
    CardDrawMutation(i32),
    CardPlayMutation(CardId),
    PassMutation,
}

const STEEL_VALUE: i32 = 2;
const TITANIUM_VALUE: i32 = 3;

impl NormalMutation {
    pub(crate) fn apply(&self, game_data: &mut GameData) -> Result<(), ()> {
        let game_data_clone = game_data.clone();
        if self.unsafe_apply(game_data) == Err(()) {
            *game_data = game_data_clone;
            return Err(());
        }
        Ok(())
    }

    fn unsafe_apply(&self, game_data: &mut GameData) -> Result<(), ()> {
        match self {
            NormalMutation::CompositeMutation(mutations) => {
                for mutation in mutations {
                    mutation.unsafe_apply(game_data)?;
                }
            }
            NormalMutation::ProductionMutation(resource, amount) => {
                let production_value = game_data.productions
                    .get_mut(&resource)
                    .expect("Production should be initialized in Game struct");
                *production_value += amount;
                if *production_value < Self::minimum_production_value_of(&resource) {
                    return Err(());
                }
            }
            NormalMutation::ResourceMutation(resource, amount) => {
                let resource_value = game_data.resources
                    .get_mut(&resource)
                    .expect("Resource should be initialized in Game struct");
                *resource_value -= amount;
                if *resource_value < 0 { return Err(()); }
            }
            NormalMutation::TrMutation(amount) => Self::increase_tr(game_data, *amount),
            NormalMutation::TemperatureIncreaseMutation => {
                if game_data.temperature < MAX_TEMPERATURE {
                    game_data.temperature += 2;
                    Self::increase_tr(game_data, 1);
                }
            }
            NormalMutation::OxygenIncreaseMutation => {
                Self::increase_oxygen_if_not_maxed_out(game_data);
            }
            NormalMutation::VictoryPointMutation(amount) => game_data.victory_points += amount,
            NormalMutation::TileQueuingMutation(tile) => {
                if game_data.board.can_place(&tile) {
                    game_data.tile_stack.push(tile.clone());
                }
            }
            NormalMutation::TagMutation(tag) => {
                *game_data.tags.get_mut(&tag).expect("Tag should be in the map") += 1;
            }
            NormalMutation::CardDrawMutation(amount) => {
                Self::draw_cards(game_data, *amount);
            }
            NormalMutation::CardPlayMutation(card_id) => {
                if game_data.cards_in_hand.remove(card_id) {
                    game_data.played_cards.insert(*card_id);
                } else {
                    return Err(());
                }
            }
            NormalMutation::BuilderCardPaymentMutation(cost) => {
                Self::mixed_payment(game_data, *cost, Steel, STEEL_VALUE)?;
            }
            NormalMutation::SpaceCardPaymentMutation(cost) => {
                Self::mixed_payment(game_data, *cost, Titanium, TITANIUM_VALUE)?;
            }
            NormalMutation::PassMutation => {
                game_data.generation += 1;

                *game_data.resource(&MegaCredit) += game_data.tr;
                *game_data.resource(&Heat) += *game_data.resource(&Energy);
                *game_data.resource(&Energy) = 0;
                for resource in [MegaCredit, Steel, Titanium, Plant, Energy, Heat] {
                    let production = *game_data.productions.get(&resource).unwrap();
                    *game_data.resource(&resource) += production;
                }

                Self::draw_cards(game_data, 4);
            }
        }
        Ok(())
    }

    fn minimum_production_value_of(resource: &Resource) -> i32 {
        match resource {
            MegaCredit => -5,
            _ => 0
        }
    }

    fn draw_cards(game_data: &mut GameData, count: i32) {
        for _ in 0..count {
            let card_id = game_data.cards_to_be_drawn.pop()
                .expect("The draw deck shouldn't run out");
            game_data.cards_in_hand.insert(card_id);
        }
    }

    pub(crate) fn increase_oxygen_if_not_maxed_out(game_data: &mut GameData) {
        if game_data.oxygen < MAX_OXYGEN {
            game_data.oxygen += 1;
            Self::increase_tr(game_data, 1);
        }
    }

    fn increase_tr(game_data: &mut GameData, amount: i32) {
        game_data.tr += amount;
        game_data.victory_points += amount;
    }

    fn mixed_payment(game_data: &mut GameData, cost: i32, resource: Resource, resource_value: i32) -> Result<(), ()> {
        let optimal_resource_cost = cost / resource_value;
        let resource_amount = *game_data.resources.get(&resource).unwrap();

        let resource_cost = min(optimal_resource_cost, resource_amount);
        let megacredit_cost = cost - resource_cost * resource_value;
        Self::spend_resource_unchecked(game_data, &resource, resource_cost);

        if *game_data.resources.get(&MegaCredit).unwrap() < megacredit_cost {
            if *game_data.resources.get(&resource).unwrap() > 0 {
                Self::spend_resource_unchecked(game_data, &resource, 1);
            } else {
                return Err(());
            }
        } else {
            Self::spend_resource_unchecked(game_data, &MegaCredit, megacredit_cost);
        }
        Ok(())
    }

    fn spend_resource_unchecked(game_data: &mut GameData, resource: &Resource, amount: i32) {
        *game_data.resources.get_mut(resource).unwrap() -= amount;
    }
}