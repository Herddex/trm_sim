use normal_mutation::NormalMutation;

use crate::model::game_data::GameData;
use crate::model::game_data::board::tile::{Tile, Tile::Greenery};

pub(crate) mod normal_mutation;

pub enum Mutation {
    Normal(NormalMutation),
    TilePlacing(Tile, (usize, usize)),
}

impl Mutation {
    pub fn apply(&self, game_data: &mut GameData) -> Result<(), ()> {
        match self {
            Mutation::TilePlacing(tile, position) => {
                if Some(tile) == game_data.tile_stack.last() {
                    game_data.board.place_tile(&tile, *position)?;
                    game_data.tile_stack.pop();
                    if *tile == Greenery {
                        NormalMutation::increase_oxygen_if_not_maxed_out(game_data);
                    }
                    while let Some(next_tile) = game_data.tile_stack.last() {
                        if !game_data.board.can_place(next_tile) {
                            game_data.tile_stack.pop();
                        } else {
                            break;
                        }
                    }
                    Ok(())
                } else {
                    Err(())
                }
            }
            Mutation::Normal(normal_mutation) => {
                if !game_data.tile_stack.is_empty() {
                    return Err(());
                }
                normal_mutation.apply(game_data)
            }
        }
    }
}