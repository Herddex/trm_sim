pub use action::invalid_action::{ActionResult, InvalidActionError};
pub use action::Action;
pub use model::card::card_compendium::ALL_CARD_IDS_IN_ASCENDING_ORDER;
pub use model::card::CardId;
pub use model::game::board::game_map::THARSIS;
pub use model::game::Game;
pub use model::game::{
    INITIAL_MEGA_CREDITS, INITIAL_OXYGEN, INITIAL_PRODUCTION, INITIAL_TEMPERATURE, INITIAL_TR,
    LAST_GENERATION, MAX_OCEANS, MAX_OXYGEN, MAX_TEMPERATURE,
};
pub use model::resource::RESOURCES;

mod action;
mod model;
