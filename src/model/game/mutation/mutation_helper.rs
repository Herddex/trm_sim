use crate::model::card::CardId;
use crate::model::game::board::tile::Tile;
use crate::model::game::board::BoardPosition;
use crate::model::game::{Game, MAX_OXYGEN, MAX_TEMPERATURE};
use crate::action::invalid_action::{ActionResult, InvalidActionError};
use crate::model::resource::Resource;
use crate::model::resource::Resource::*;
use std::cmp::min;

pub fn minimum_production_value_of(resource: &Resource) -> i32 {
    match resource {
        MegaCredit => -5,
        _ => 0,
    }
}

pub fn draw_cards(game: &mut Game, count: i32) {
    for _ in 0..count {
        let card_id = game
            .cards_to_be_drawn
            .pop()
            .expect("The draw deck shouldn't run out");
        game.cards_in_hand.insert(card_id);
    }
}

pub fn increase_oxygen_if_not_maxed_out(game: &mut Game, mut amount: u32) {
    while game.oxygen < MAX_OXYGEN && amount > 0 {
        game.oxygen += 1;
        increase_tr(game, 1);
        amount -= 1;
    }
}

pub fn increase_tr(game: &mut Game, amount: i32) {
    game.tr += amount;
    game.victory_points += amount;
}

pub fn mixed_payment(
    game: &mut Game,
    cost: i32,
    resource: Resource,
    resource_value: i32,
) -> ActionResult {
    let optimal_resource_cost = cost / resource_value;
    let resource_amount = *game.resources.get(&resource).unwrap();

    let resource_cost = min(optimal_resource_cost, resource_amount);
    let megacredit_cost = cost - resource_cost * resource_value;
    spend_resource_unchecked(game, &resource, resource_cost);

    if *game.resources.get(&MegaCredit).unwrap() < megacredit_cost {
        if *game.resources.get(&resource).unwrap() > 0 {
            spend_resource_unchecked(game, &resource, 1);
        } else {
            return Err(InvalidActionError::new(format!(
                "Insufficient {:?} and Mega Credits.",
                resource
            )));
        }
    } else {
        spend_resource_unchecked(game, &MegaCredit, megacredit_cost);
    }
    Ok(())
}

fn spend_resource_unchecked(game: &mut Game, resource: &Resource, amount: i32) {
    *game.resources.get_mut(resource).unwrap() -= amount;
}

pub fn pass(game: &mut Game) {
    game.generation += 1;

    *game.resource_mut(&MegaCredit) += game.tr;
    *game.resource_mut(&Heat) += *game.resource_mut(&Energy);
    *game.resource_mut(&Energy) = 0;
    for resource in [MegaCredit, Steel, Titanium, Plant, Energy, Heat] {
        let production = *game.productions.get(&resource).unwrap();
        *game.resource_mut(&resource) += production;
    }

    draw_cards(game, 4);
}

pub fn place_tile(game: &mut Game, position: BoardPosition) -> ActionResult {
    if game.tile_stack.is_empty() {
        InvalidActionError::from("No tile to place").into_err()
    } else {
        let tile = *game.tile_stack.last().unwrap();
        game.board.place_tile(&tile, position)?;
        game.tile_stack.pop();

        match tile {
            Tile::Greenery => increase_oxygen_if_not_maxed_out(game, 1),
            Tile::Ocean => increase_tr(game, 1),
            _ => (),
        }

        let tile_stack = &mut game.tile_stack;
        while !tile_stack.is_empty() && !game.board.can_place(tile_stack.last().unwrap()) {
            tile_stack.pop();
        }

        Ok(())
    }
}

pub fn production_change(game: &mut Game, resource: &Resource, delta: i32) -> ActionResult {
    let production_value = game
        .productions
        .get_mut(resource)
        .expect("Production should be initialized in Game struct");
    *production_value += delta;
    let min_val = minimum_production_value_of(resource);
    if *production_value < min_val {
        InvalidActionError::new(format!(
            "{:?} production cannot be lower than {}",
            resource, min_val
        ))
        .into_err()
    } else {
        Ok(())
    }
}

pub fn resource_change(game: &mut Game, resource: &Resource, delta: i32) -> ActionResult {
    let resource_value = game
        .resources
        .get_mut(resource)
        .expect("Resource should be initialized in Game struct");
    *resource_value += delta;
    if *resource_value < 0 {
        InvalidActionError::new(format!("Not enough {:?} resources", resource)).into_err()
    } else {
        Ok(())
    }
}

pub fn increase_temperature_if_not_maxed_out(game: &mut Game, mut amount: u32) {
    while game.temperature < MAX_TEMPERATURE && amount > 0 {
        game.temperature += 2;
        amount -= 1;
        increase_tr(game, 1);
    }
}

pub fn play_card(game: &mut Game, card_id: CardId) -> ActionResult {
    if game.cards_in_hand.remove(&card_id) {
        game.played_cards.insert(card_id);
        Ok(())
    } else {
        InvalidActionError::new(format!("Card #{:0>3} not in hand", card_id)).into_err()
    }
}
