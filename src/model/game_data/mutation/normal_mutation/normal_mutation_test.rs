use rstest::{fixture, rstest};
use crate::{assert_err, assert_ok};
use crate::model::game_data::{INITIAL_MEGA_CREDITS, INITIAL_PRODUCTION, INITIAL_TR};
use crate::model::game_data::board::MAX_OCEANS;
use crate::model::game_data::board::tile::Tile::Ocean;
use crate::model::tag::Tag::Power;
use super::*;
use super::NormalMutation::*;
use super::super::super::board::game_map::THARSIS;

impl GameData {
    fn apply(&mut self, normal_mutation: &NormalMutation) -> Result<(), ()> {
        normal_mutation.apply(self)
    }
}

#[fixture]
fn game_data() -> GameData {
    let mut game_data = GameData::new(&THARSIS);
    game_data.cards_to_be_drawn.sort_unstable();
    game_data
}

#[rstest]
fn test_production_mutations(mut game_data: GameData) {
    assert_err!(game_data.apply(&ProductionMutation(MegaCredit, - INITIAL_PRODUCTION - 6)));
    assert_ok!(game_data.apply(&ProductionMutation(MegaCredit, - INITIAL_PRODUCTION - 5)));
    assert_err!(game_data.apply(&ProductionMutation(MegaCredit, -1)));

    assert_err!(game_data.apply(&ProductionMutation(Energy, - INITIAL_PRODUCTION - 1)));
    assert_ok!(game_data.apply(&ProductionMutation(Energy, 1)));
    assert_ok!(game_data.apply(&ProductionMutation(Energy, - INITIAL_PRODUCTION - 1)));
}

#[rstest]
#[case(25, Steel, STEEL_VALUE, BuilderCardPaymentMutation(25))]
#[case(25, Titanium, TITANIUM_VALUE, SpaceCardPaymentMutation(25))]
fn test_mixed_payment(mut game_data: GameData, #[case] cost: i32, #[case] resource: Resource,
                      #[case] resource_value: i32, #[case] mutation: NormalMutation) {
    *game_data.resource(&MegaCredit) = 0;
    *game_data.resource(&resource) += cost / resource_value;
    assert_err!(game_data.apply(&mutation));
    *game_data.resource(&MegaCredit) += cost % resource_value;
    assert_ok!(game_data.apply(&mutation));
    assert_eq!(*game_data.resource(&MegaCredit), 0);
    assert_eq!(*game_data.resource(&resource), 0);

    *game_data.resource(&MegaCredit) += cost - 1;
    assert_err!(game_data.apply(&mutation));
    *game_data.resource(&MegaCredit) += 1;
    assert_ok!(game_data.apply(&mutation));
    assert_eq!(*game_data.resource(&MegaCredit), 0);

    *game_data.resource(&resource) += cost / resource_value + 1;
    assert_ok!(game_data.apply(&mutation));
    assert_eq!(*game_data.resource(&resource), 0);
    assert_eq!(*game_data.resource(&MegaCredit), 0);

    *game_data.resource(&resource) += cost / resource_value + 1;
    *game_data.resource(&MegaCredit)
        += cost % resource_value + 15;
    assert_ok!(game_data.apply(&mutation));
    assert_eq!(*game_data.resource(&resource), 1);
    assert_eq!(*game_data.resource(&MegaCredit), 15);

    *game_data.resource(&resource) += 4;
    assert_ok!(game_data.apply(&mutation));
    assert_eq!(*game_data.resource(&resource), 0);
    assert_eq!(*game_data.resource(&MegaCredit),
               15 - cost + 5 * resource_value);
}

#[rstest]
fn test_resource_mutation(mut game_data: GameData) {
    assert_err!(game_data.apply(&ResourceMutation(MegaCredit, INITIAL_MEGA_CREDITS + 1)));
    assert_ok!(game_data.apply(&ResourceMutation(MegaCredit, INITIAL_MEGA_CREDITS)));
    assert_ok!(game_data.apply(&ResourceMutation(MegaCredit, 0)));
}

#[rstest]
fn test_temperature_and_oxygen_increase_mutations(mut game_data: GameData) {
    let previous_tr = game_data.tr;
    game_data.temperature = -16;
    game_data.oxygen = 3;

    assert_ok!(game_data.apply(&TemperatureIncreaseMutation));
    assert_ok!(game_data.apply(&TemperatureIncreaseMutation));
    assert_ok!(game_data.apply(&TemperatureIncreaseMutation));
    assert_ok!(game_data.apply(&OxygenIncreaseMutation));
    assert_ok!(game_data.apply(&OxygenIncreaseMutation));

    assert_eq!(game_data.tr, previous_tr + 5);
    assert_eq!(game_data.temperature, -10);
    assert_eq!(game_data.oxygen, 5);

    game_data.temperature = MAX_TEMPERATURE - 4;
    game_data.oxygen = MAX_OXYGEN - 1;
    assert_ok!(game_data.apply(&OxygenIncreaseMutation));
    assert_ok!(game_data.apply(&OxygenIncreaseMutation));
    assert_ok!(game_data.apply(&TemperatureIncreaseMutation));
    assert_ok!(game_data.apply(&TemperatureIncreaseMutation));
    assert_ok!(game_data.apply(&TemperatureIncreaseMutation));

    assert_eq!(game_data.tr, previous_tr + 8);
    assert_eq!(game_data.temperature, MAX_TEMPERATURE);
    assert_eq!(game_data.oxygen, MAX_OXYGEN);
}

#[rstest]
fn test_tag_mutation(mut game_data: GameData) {
    let tag_count = *game_data.tags.get(&Power).unwrap();
    assert_ok!(game_data.apply(&TagMutation(Power)));
    assert_eq!(tag_count + 1, *game_data.tags.get(&Power).unwrap());
}

#[rstest]
fn test_card_draw(mut game_data: GameData) {
    let top_card = *game_data.cards_to_be_drawn.last().unwrap();
    assert_ok!(game_data.apply(&CardDrawMutation(1)));
    assert!(game_data.cards_in_hand.contains(&top_card));
    assert!(!game_data.cards_to_be_drawn.contains(&top_card));
}

#[rstest]
fn test_card_play(mut game_data: GameData) {
    let card_id = *game_data.cards_in_hand.iter().take(1).last().unwrap();
    assert_ok!(game_data.apply(&CardPlayMutation(card_id)));
    assert!(!game_data.cards_in_hand.contains(&card_id));
    assert!(game_data.played_cards.contains(&card_id));

    assert_err!(game_data.apply(&CardPlayMutation(24353453)));
}

#[rstest]
fn test_tr_mutation(mut game_data: GameData) {
    let prev_tr = game_data.tr;
    let prev_vp = game_data.victory_points;

    assert_ok!(game_data.apply(&TrMutation(2)));
    assert_eq!(game_data.tr, prev_tr + 2);
    assert_eq!(game_data.victory_points, prev_vp + 2);
}

#[rstest]
fn test_vp_mutation(mut game_data: GameData) {
    let prev_vp = game_data.victory_points;

    assert_ok!(game_data.apply(&VictoryPointMutation(2)));
    assert_eq!(game_data.victory_points, prev_vp + 2);
}

#[rstest]
fn test_pass_mutation(mut game_data: GameData) {
    assert_ok!(game_data.apply(&PassMutation));

    assert_eq!(game_data.generation, 2);
    assert_eq!(*game_data.resource(&MegaCredit),
               INITIAL_MEGA_CREDITS + INITIAL_TR + INITIAL_PRODUCTION);
    for resource in [Steel, Titanium, Plant, Energy, Heat] {
        assert_eq!(*game_data.resource(&resource), 1);
    }

    *game_data.productions.get_mut(&Energy).unwrap() += 2;
    *game_data.productions.get_mut(&MegaCredit).unwrap() += 1;
    game_data.tr += 2;

    assert_ok!(game_data.apply(&PassMutation));

    assert_eq!(game_data.generation, 3);
    assert_eq!(*game_data.resource(&MegaCredit),
               INITIAL_MEGA_CREDITS + 2 * INITIAL_TR + 2 * INITIAL_PRODUCTION + 3);

    assert_eq!(*game_data.resource(&Energy), 3);
    assert_eq!(*game_data.resource(&Heat), 3);
}

#[rstest]
fn test_tile_queueing(mut game_data: GameData) {
    assert_ok!(game_data.apply(&TileQueuingMutation(Ocean)));
    assert_eq!(game_data.tile_stack, vec![Ocean]);
    for ocean_position in THARSIS.ocean_positions().iter().take(MAX_OCEANS) {
        assert_ok!(game_data.board.place_tile(&Ocean, *ocean_position));
    }
    assert_eq!(game_data.board.placed_oceans(), MAX_OCEANS);
    assert_eq!(game_data.tile_stack, vec![Ocean]);
}