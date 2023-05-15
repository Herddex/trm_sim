extern crate core;

mod model;
mod game;
mod test_util;

pub use game::{Game, action::{Action, standard_project::StandardProject}};
pub use model::game_data::board::game_map::THARSIS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_test() {
        let mut game = Game::new(&THARSIS);
        assert_eq!(game.do_action(Action::Card(3253453)), Err(()));
        assert_eq!(game.do_action(Action::StandardProject(StandardProject::Asteroid)), Ok(()));
    }
}