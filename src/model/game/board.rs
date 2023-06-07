use std::fmt::{Display, Formatter};

use game_map::GameMap;
use tile::Tile;
use tile::Tile::*;

use crate::action::invalid_action::InvalidActionError;

pub(crate) mod game_map;
pub(crate) mod tile;

pub type BoardPosition = (usize, usize);

#[derive(Clone)]
pub struct Board {
    tiles: [Vec<Tile>; 9],
    game_map: &'static GameMap,
    can_place_greenery_adjacent_to_owned_tiles: bool,
}

impl Board {
    pub fn new(map: &'static GameMap) -> Self {
        Self {
            tiles: [
                vec![Empty; 5],
                vec![Empty; 6],
                vec![Empty; 7],
                vec![Empty; 8],
                vec![Empty; 9],
                vec![Empty; 8],
                vec![Empty; 7],
                vec![Empty; 6],
                vec![Empty; 5],
            ],
            game_map: map,
            can_place_greenery_adjacent_to_owned_tiles: false,
        }
    }

    pub fn can_place(&self, tile: &Tile) -> bool {
        match tile {
            Ocean => true,
            City | Greenery => self.tiles.iter().enumerate().any(|(row, tile_row)| {
                tile_row
                    .iter()
                    .enumerate()
                    .any(|(column, _)| self.is_valid_placement(tile, row, column))
            }),
            _ => false,
        }
    }

    pub fn place_tile(
        &mut self,
        tile: &Tile,
        position: BoardPosition,
    ) -> Result<i32, InvalidActionError> {
        if !self.is_valid_placement(tile, position.0, position.1) {
            return Err(InvalidActionError::new(format!(
                "{:?} cannot be placed on position {:?}",
                tile, position
            )));
        }

        self.tiles[position.0][position.1] = *tile;
        if Self::is_owned_tile(tile) {
            self.can_place_greenery_adjacent_to_owned_tiles = true
        }

        Ok(match *tile {
            City => {
                Self::neighbour_positions_of(position.0, position.1)
                    .into_iter()
                    .filter(|position| self.tiles[position.0][position.1] == Greenery)
                    .count() as i32
            }
            Greenery => {
                1 + Self::neighbour_positions_of(position.0, position.1)
                    .into_iter()
                    .filter(|position| self.tiles[position.0][position.1] == City)
                    .count() as i32
            }
            _ => 0,
        })
    }

    fn is_valid_placement(&self, tile: &Tile, row: usize, column: usize) -> bool {
        if Self::is_invalid_position(row, column) || self.tiles[row][column] != Empty {
            return false;
        }

        if self.game_map.is_ocean_position((row, column)) {
            return *tile == Ocean;
        }

        match tile {
            City => !self.city_exists_around(row, column),
            Greenery => {
                !self.can_place_greenery_adjacent_to_owned_tiles
                    || self.has_owned_tiles_around(row, column)
            }
            _ => false,
        }
    }

    fn city_exists_around(&self, row: usize, column: usize) -> bool {
        Self::neighbour_positions_of(row, column)
            .iter()
            .any(|(row, column)| self.tiles[*row][*column] == City)
    }

    fn has_owned_tiles_around(&self, row: usize, column: usize) -> bool {
        Self::neighbour_positions_of(row, column)
            .iter()
            .any(|(row, column)| Self::is_owned_tile(&self.tiles[*row][*column]))
    }

    fn is_owned_tile(tile: &Tile) -> bool {
        matches!(tile, Greenery | City)
    }

    fn neighbour_positions_of(row: usize, column: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::with_capacity(6);

        if (1..=4).contains(&row) && column < row + 4 {
            neighbours.push((row - 1, column));
        }
        if row >= 5 {
            neighbours.push((row - 1, column + 1));
        }

        if row <= 4 && column < row + 4 || row >= 5 && column < 12 - row {
            neighbours.push((row, column + 1));
        }

        if row <= 3 {
            neighbours.push((row + 1, column + 1));
        }
        if row >= 4 || row <= 7 && column < 12 - row {
            neighbours.push((row + 1, column));
        }

        if row <= 3 {
            neighbours.push((row + 1, column));
        }
        if (4..=7).contains(&row) && column > 0 {
            neighbours.push((row + 1, column - 1));
        }

        if column > 0 {
            neighbours.push((row, column - 1));
        }

        if (1..=4).contains(&row) && column > 0 {
            neighbours.push((row - 1, column - 1));
        }
        if row >= 5 {
            neighbours.push((row - 1, column))
        }

        neighbours
    }

    fn is_invalid_position(row: usize, column: usize) -> bool {
        row > 8 || row <= 4 && column > row + 4 || row > 4 && column > 12 - row
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (row, row_tiles) in self.tiles.iter().enumerate() {
            let indent = row.abs_diff(4);
            for _ in 0..indent {
                write!(f, " ")?;
            }
            for (column, tile) in row_tiles.iter().enumerate() {
                if *tile == Empty && self.game_map.is_ocean_position((row, column)) {
                    write!(f, "_ ")?;
                } else {
                    write!(f, "{} ", *tile)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use lazy_static::lazy_static;

    use crate::THARSIS;

    use super::*;

    lazy_static! {
        static ref EMPTY_THARSIS_BOARD: Board = Board::new(&THARSIS);
        static ref BOARD_POSITIONS: HashSet<(usize, usize)> = {
            let mut board_positions = HashSet::new();
            EMPTY_THARSIS_BOARD
                .tiles
                .iter()
                .enumerate()
                .for_each(|(row, row_tiles)| {
                    row_tiles.iter().enumerate().for_each(|(column, _)| {
                        board_positions.insert((row, column));
                    })
                });
            board_positions
        };
    }

    #[test]
    fn test_can_place_oceans() {
        BOARD_POSITIONS.iter().for_each(|position| {
            assert_eq!(
                THARSIS.is_ocean_position(*position),
                EMPTY_THARSIS_BOARD.is_valid_placement(&Ocean, position.0, position.1),
                "Ocean placement test on position ({}, {}) failed.",
                position.0,
                position.1
            );
        })
    }

    #[test]
    fn test_city_placements() {
        let mut board = Board::new(&THARSIS);
        assert!(board.place_tile(&City, (0, 0)).is_ok());
        assert!(board.place_tile(&City, (1, 0)).is_err());
        assert!(board.place_tile(&City, (1, 1)).is_err());

        assert!(board.place_tile(&City, (2, 1)).is_ok());

        assert!(board.place_tile(&City, (4, 0)).is_ok());
        assert!(board.place_tile(&City, (5, 1)).is_ok());
    }

    #[test]
    fn test_greenery_placements() {
        let mut board = Board::new(&THARSIS);
        assert!(board.place_tile(&City, (4, 0)).is_ok());
        assert!(board.place_tile(&Greenery, (4, 2)).is_err());
        assert!(board.place_tile(&Greenery, (3, 0)).is_ok());
        assert!(board.place_tile(&Greenery, (4, 1)).is_ok());
        assert!(board.place_tile(&Greenery, (5, 0)).is_ok());
    }
}
