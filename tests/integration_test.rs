use trm_sim::*;

#[test]
fn standard_asteroid_test() {
    let mut game = Game::new(&THARSIS);
    let mut temperature_increases = 0;

    while !game.is_over() {
        while game.temperature() < MAX_TEMPERATURE
            && Action::StandardAsteroid.execute(&mut game).is_ok()
        {
            temperature_increases += 1;
        }
        assert!(Action::Pass.execute(&mut game).is_ok());
    }

    assert_eq!(
        game.temperature() - INITIAL_TEMPERATURE,
        temperature_increases * 2
    );
}
