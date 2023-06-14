mod mutation_helper;

use std::fmt::{Display, Formatter};

use crate::action::invalid_action::{ActionResult, InvalidActionError};
use crate::model::card::CardId;
use crate::model::game::board::tile::Tile;
use crate::model::game::Game;
use crate::model::resource::Resource;
use crate::model::resource::Resource::*;
use crate::model::tag::Tag;

#[derive(Clone)]
pub(crate) enum Mutation {
    Composite(Vec<Mutation>),
    Production(Resource, i32),
    Resource(Resource, i32),
    BuilderCardPayment(i32),
    SpaceCardPayment(i32),
    TR(i32),
    TemperatureIncrease(i32),
    OxygenIncrease(i32),
    VictoryPoint(i32),
    TilePlacement(Tile),
    Tag(Tag),
    CardDraw(i32),
    CardPlay(CardId),
    Pass,
}

const STEEL_VALUE: i32 = 2;
const TITANIUM_VALUE: i32 = 3;

impl Mutation {
    pub(crate) fn apply(&self, game: &mut Game) -> ActionResult {
        if game.is_over() {
            return InvalidActionError::from("Game is over").into_err();
        }

        let game_clone = game.clone();
        let result = self.unsafe_apply(game);
        if result.is_err() {
            *game = game_clone;
        }

        result
    }

    fn unsafe_apply(&self, game: &mut Game) -> ActionResult {
        match self {
            Mutation::Composite(mutations) => {
                for mutation in mutations {
                    mutation.unsafe_apply(game)?;
                }
            }
            Mutation::Production(resource, delta) => {
                mutation_helper::production_change(game, resource, *delta)?
            }
            Mutation::Resource(resource, delta) => {
                mutation_helper::resource_change(game, resource, *delta)?
            }
            Mutation::TR(amount) => mutation_helper::increase_tr(game, *amount),
            Mutation::TemperatureIncrease(amount) => {
                mutation_helper::increase_temperature_if_not_maxed_out(game, *amount)
            }
            Mutation::OxygenIncrease(amount) => {
                mutation_helper::increase_oxygen_if_not_maxed_out(game, *amount)
            }
            Mutation::VictoryPoint(amount) => game.victory_points += amount,
            Mutation::TilePlacement(tile) => mutation_helper::place_tile_greedily(game, tile),
            Mutation::Tag(tag) => *game.tags.get_mut(tag).expect("Tag should be in the map") += 1,
            Mutation::CardDraw(amount) => mutation_helper::draw_cards(game, *amount),
            Mutation::CardPlay(card_id) => mutation_helper::play_card(game, *card_id)?,
            Mutation::BuilderCardPayment(cost) => {
                mutation_helper::mixed_payment(game, *cost, Steel, STEEL_VALUE)?
            }
            Mutation::SpaceCardPayment(cost) => {
                mutation_helper::mixed_payment(game, *cost, Titanium, TITANIUM_VALUE)?
            }
            Mutation::Pass => mutation_helper::pass(game),
        }
        Ok(())
    }
}

impl Display for Mutation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mutation::Composite(mutations) => {
                for mutation in mutations {
                    writeln!(f, "{}", mutation)?;
                }
            }
            Mutation::Production(resource, delta) => {
                write!(f, "{} {:?} production(s)", delta, resource)?
            }
            Mutation::Resource(resource, delta) => {
                write!(f, "{} {:?} resource(s)", delta, resource)?
            }
            Mutation::BuilderCardPayment(cost) => write!(f, "Cost (Steel can be used): {}", cost)?,
            Mutation::SpaceCardPayment(cost) => write!(f, "Cost (Titanium can be used): {}", cost)?,
            Mutation::TR(amount) => write!(f, "{} TR", amount)?,
            Mutation::TemperatureIncrease(amount) => {
                write!(f, "Increase temperature {} step(s)", amount)?
            }
            Mutation::OxygenIncrease(amount) => write!(f, "Increase oxygen {} step(s)", amount)?,
            Mutation::VictoryPoint(amount) => write!(f, "{} victory point(s)", amount)?,
            Mutation::TilePlacement(tile) => write!(f, "Place tile: {:?}", tile)?,
            Mutation::Tag(tag) => write!(f, "{:?} Tag", tag)?,
            Mutation::CardDraw(amount) => write!(f, "Draw {} cards", amount)?,
            Mutation::CardPlay(card_id) => write!(f, "Card #{}", *card_id)?,
            Mutation::Pass => write!(f, "Pass")?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::game::{
        INITIAL_MEGA_CREDITS, INITIAL_PRODUCTION, INITIAL_TR, MAX_OXYGEN, MAX_TEMPERATURE,
    };
    use crate::model::resource::Resource;
    use crate::model::tag::Tag::Power;
    use crate::THARSIS;
    use rstest::{fixture, rstest};

    impl Game {
        fn apply(&mut self, mutation: &Mutation) -> ActionResult {
            mutation.apply(self)
        }
    }

    #[fixture]
    fn game() -> Game {
        let mut game = Game::new(&THARSIS);
        game.cards_to_be_drawn.sort_unstable();
        game
    }

    #[rstest]
    fn test_production_mutations(mut game: Game) {
        assert!(game
            .apply(&Mutation::Production(MegaCredit, -INITIAL_PRODUCTION - 6))
            .is_err());
        assert!(game
            .apply(&Mutation::Production(MegaCredit, -INITIAL_PRODUCTION - 5))
            .is_ok());
        assert!(game.apply(&Mutation::Production(MegaCredit, -1)).is_err());

        assert!(game
            .apply(&Mutation::Production(Energy, -INITIAL_PRODUCTION - 1))
            .is_err());
        assert!(game.apply(&Mutation::Production(Energy, 1)).is_ok());
        assert!(game
            .apply(&Mutation::Production(Energy, -INITIAL_PRODUCTION - 1))
            .is_ok());
    }

    #[rstest]
    #[case(25, Steel, STEEL_VALUE, Mutation::BuilderCardPayment(25))]
    #[case(25, Titanium, TITANIUM_VALUE, Mutation::SpaceCardPayment(25))]
    fn test_mixed_payment(
        mut game: Game,
        #[case] cost: i32,
        #[case] resource: Resource,
        #[case] resource_value: i32,
        #[case] mutation: Mutation,
    ) {
        *game.resource_mut(&MegaCredit) = 0;
        *game.resource_mut(&resource) += cost / resource_value;
        assert!(game.apply(&mutation).is_err());
        *game.resource_mut(&MegaCredit) += cost % resource_value;
        assert!(game.apply(&mutation).is_ok());
        assert_eq!(*game.resource_mut(&MegaCredit), 0);
        assert_eq!(*game.resource_mut(&resource), 0);

        *game.resource_mut(&MegaCredit) += cost - 1;
        assert!(game.apply(&mutation).is_err());
        *game.resource_mut(&MegaCredit) += 1;
        assert!(game.apply(&mutation).is_ok());
        assert_eq!(*game.resource_mut(&MegaCredit), 0);

        *game.resource_mut(&resource) += cost / resource_value + 1;
        assert!(game.apply(&mutation).is_ok());
        assert_eq!(*game.resource_mut(&resource), 0);
        assert_eq!(*game.resource_mut(&MegaCredit), 0);

        *game.resource_mut(&resource) += cost / resource_value + 1;
        *game.resource_mut(&MegaCredit) += cost % resource_value + 15;
        assert!(game.apply(&mutation).is_ok());
        assert_eq!(*game.resource_mut(&resource), 1);
        assert_eq!(*game.resource_mut(&MegaCredit), 15);

        *game.resource_mut(&resource) += 4;
        assert!(game.apply(&mutation).is_ok());
        assert_eq!(*game.resource_mut(&resource), 0);
        assert_eq!(
            *game.resource_mut(&MegaCredit),
            15 - cost + 5 * resource_value
        );
    }

    #[rstest]
    fn test_resource_mutation(mut game: Game) {
        assert!(game
            .apply(&Mutation::Resource(MegaCredit, -INITIAL_MEGA_CREDITS - 1))
            .is_err());
        assert!(game
            .apply(&Mutation::Resource(MegaCredit, -INITIAL_MEGA_CREDITS))
            .is_ok());
        assert!(game.apply(&Mutation::Resource(MegaCredit, 0)).is_ok());
    }

    #[rstest]
    fn test_temperature_and_oxygen_increase_mutations(mut game: Game) {
        let previous_tr = game.tr;
        game.temperature = -16;
        game.oxygen = 3;

        assert!(game.apply(&Mutation::TemperatureIncrease(3)).is_ok());
        assert!(game.apply(&Mutation::OxygenIncrease(2)).is_ok());

        assert_eq!(game.tr, previous_tr + 5);
        assert_eq!(game.temperature, -10);
        assert_eq!(game.oxygen, 5);

        game.temperature = MAX_TEMPERATURE - 4;
        game.oxygen = MAX_OXYGEN - 1;
        assert!(game.apply(&Mutation::OxygenIncrease(2)).is_ok());
        assert!(game.apply(&Mutation::TemperatureIncrease(3)).is_ok());

        assert_eq!(game.tr, previous_tr + 8);
        assert_eq!(game.temperature, MAX_TEMPERATURE);
        assert_eq!(game.oxygen, MAX_OXYGEN);
    }

    #[rstest]
    fn test_tag_mutation(mut game: Game) {
        let tag_count = *game.tags.get(&Power).unwrap();
        assert!(game.apply(&Mutation::Tag(Power)).is_ok());
        assert_eq!(tag_count + 1, *game.tags.get(&Power).unwrap());
    }

    #[rstest]
    fn test_card_draw(mut game: Game) {
        let top_card = *game.cards_to_be_drawn.last().unwrap();
        assert!(game.apply(&Mutation::CardDraw(1)).is_ok());
        assert!(game.cards_in_hand.contains(&top_card));
        assert!(!game.cards_to_be_drawn.contains(&top_card));
    }

    #[rstest]
    fn test_card_play(mut game: Game) {
        let card_id = *game.cards_in_hand.iter().take(1).last().unwrap();
        assert!(game.apply(&Mutation::CardPlay(card_id)).is_ok());
        assert!(!game.cards_in_hand.contains(&card_id));
        assert!(game.played_cards.contains(&card_id));

        assert!(game.apply(&Mutation::CardPlay(24353453)).is_err());
    }

    #[rstest]
    fn test_tr_mutation(mut game: Game) {
        let prev_tr = game.tr;
        let prev_vp = game.victory_points;

        assert!(game.apply(&Mutation::TR(2)).is_ok());
        assert_eq!(game.tr, prev_tr + 2);
        assert_eq!(game.victory_points, prev_vp + 2);
    }

    #[rstest]
    fn test_vp_mutation(mut game: Game) {
        let prev_vp = game.victory_points;

        assert!(game.apply(&Mutation::VictoryPoint(2)).is_ok());
        assert_eq!(game.victory_points, prev_vp + 2);
    }

    #[rstest]
    fn test_pass_mutation(mut game: Game) {
        assert!(game.apply(&Mutation::Pass).is_ok());

        assert_eq!(game.generation, 2);
        assert_eq!(
            *game.resource_mut(&MegaCredit),
            INITIAL_MEGA_CREDITS + INITIAL_TR + INITIAL_PRODUCTION
        );
        for resource in [Steel, Titanium, Plant, Energy, Heat] {
            assert_eq!(*game.resource_mut(&resource), 1);
        }

        *game.productions.get_mut(&Energy).unwrap() += 2;
        *game.productions.get_mut(&MegaCredit).unwrap() += 1;
        game.tr += 2;

        assert!(game.apply(&Mutation::Pass).is_ok());

        assert_eq!(game.generation, 3);
        assert_eq!(
            *game.resource_mut(&MegaCredit),
            INITIAL_MEGA_CREDITS + 2 * INITIAL_TR + 2 * INITIAL_PRODUCTION + 3
        );

        assert_eq!(*game.resource_mut(&Energy), 3);
        assert_eq!(*game.resource_mut(&Heat), 3);
    }
}
