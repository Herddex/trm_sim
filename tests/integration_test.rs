use trm_sim::*;

#[test]
fn card_test() {
    let mut game = Game::new(&THARSIS);
    assert!(Action::Card(3253453).execute(&mut game).is_err());
    assert!(Action::StandardAsteroid.execute(&mut game).is_ok());
}

#[test]
fn city_test() {
    let mut game = Game::new(&THARSIS);
    let game = &mut game;
    assert!(Action::StandardCity.execute(game).is_ok());
    assert!(Action::TilePlacement((4, 1)).execute(game).is_ok());
    assert!(Action::Pass.execute(game).is_ok());
    assert!(Action::StandardCity.execute(game).is_ok());
    assert!(Action::TilePlacement((5, 0)).execute(game).is_err());
    assert!(Action::TilePlacement((5, 1)).execute(game).is_err());
    assert!(Action::TilePlacement((5, 2)).execute(game).is_ok());
}
