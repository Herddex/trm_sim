use std::io;
use trm_sim::*;

fn parse_tile_position_input(possible_tile_position_input: &str) -> Option<Action> {
    let mut split = possible_tile_position_input.split_whitespace();
    let row = split.next()?.parse::<usize>().ok()?;
    let column = split.next()?.parse::<usize>().ok()?;
    Some(Action::TilePlacement((row, column)))
}

fn map_user_input(input: &str) -> Result<Action, std::fmt::Error> {
    if let Ok(card_id) = input.parse::<CardId>() {
        return Ok(Action::Card(card_id));
    }

    match input.to_lowercase().as_str() {
        "hc" => Ok(Action::HeatConversion),
        "pc" => Ok(Action::PlantConversion),
        "sc" => Ok(Action::StandardCity),
        "sg" => Ok(Action::StandardGreenery),
        "st" => Ok(Action::StandardAsteroid),
        "so" => Ok(Action::StandardAquifer),
        "se" => Ok(Action::StandardPowerPlant),
        "pass" => Ok(Action::Pass),
        value => {
            if let Some(tile_placement) = parse_tile_position_input(value) {
                Ok(tile_placement)
            } else {
                Err(std::fmt::Error::default())
            }
        }
    }
}

fn main() {
    let mut game = Game::new(&THARSIS);
    let mut user_input = String::new();
    println!("{}", game);

    loop {
        user_input.clear();
        if let Err(error) = io::stdin().read_line(&mut user_input) {
            println!("Error: {}", error);
            continue;
        }

        let input = user_input.trim();
        if input.eq_ignore_ascii_case("quit") {
            return;
        }

        let action = match map_user_input(input) {
            Err(error) => {
                println!("Invalid input: {}", error);
                continue;
            }
            Ok(action) => action,
        };

        if let Err(error) = action.execute(&mut game) {
            println!("Invalid action: {}", error);
            continue;
        } else if game.is_over() {
            println!("{}", game);
            println!(
                "Game over. {}.",
                if game.is_won() {
                    "You won!"
                } else {
                    "You lost."
                }
            );
            return;
        }

        println!("{}", game);
    }
}
