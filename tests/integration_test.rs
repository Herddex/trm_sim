use trm_sim::*;

#[test]
fn card_test() {
    let mut game = Game::new(&THARSIS);
    assert!(Action::Card(3253453).execute(&mut game).is_err());
    assert!(Action::StandardAsteroid.execute(&mut game).is_ok());
}
