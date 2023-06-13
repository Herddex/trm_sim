use std::fmt::{Display, Formatter};

use game_map::GameMap;
use tile::Tile;
use tile::Tile::*;

pub(crate) mod game_map;
pub(crate) mod tile;

pub type BoardPosition = (usize, usize);
type VictoryPoints = i32;

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

    pub fn tiles(&self) -> &[Vec<Tile>; 9] {
        &self.tiles
    }

    pub fn place_tile(&mut self, tile: Tile) -> i32 {
        match tile {
            Ocean => {
                self.place_ocean_greedily();
                0
            }
            Greenery => self.place_greenery_greedily(),
            City => self.place_city_greedily(),
            Empty => 0,
        }
    }

    /**
    Invariant: The maximum number of oceans has not yet been reached
    */
    fn place_ocean_greedily(&mut self) {
        let (i, j) = self
            .game_map
            .ocean_positions()
            .iter()
            .find(|(i, j)| self.tiles[*i][*j] == Empty)
            .expect("There should be at least one empty ocean position");
        self.tiles[*i][*j] = Ocean;
    }

    fn place_greenery_greedily(&mut self) -> VictoryPoints {
        let mut legal_positions = Vec::new();
        for (i, row) in self.tiles.iter().enumerate() {
            legal_positions.extend(
                row.iter()
                    .enumerate()
                    .filter(|(j, _)| self.can_place_greenery_at((i, *j)))
                    .map(|(j, _)| (i, j)),
            );
        }

        legal_positions.sort_by(|x, y| {
            self.neighbours_of(*x, City)
                .cmp(&self.neighbours_of(*y, City))
        });

        match legal_positions.last() {
            None => 0,
            Some((i, j)) => {
                self.tiles[*i][*j] = Greenery;
                self.can_place_greenery_adjacent_to_owned_tiles = true;
                self.neighbours_of((*i, *j), City) as i32 + 1
            }
        }
    }

    fn can_place_greenery_at(&self, position: (usize, usize)) -> bool {
        self.can_place_non_ocean_tile_at(position)
            && (!self.can_place_greenery_adjacent_to_owned_tiles
                || self.has_owned_tiles_around(position.0, position.1))
    }

    /**
    Returns the victory points (not TR) earned from the tile placement
     */
    fn place_city_greedily(&mut self) -> i32 {
        let mut legal_positions = Vec::new();
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if self.can_place_city_at((i, j)) {
                    legal_positions.push((i, j));
                }
            }
        }

        legal_positions.sort_by(|x, y| {
            self.neighbours_of(*x, Greenery)
                .cmp(&self.neighbours_of(*y, Greenery))
        });

        match legal_positions.last() {
            None => 0,
            Some((i, j)) => {
                self.tiles[*i][*j] = City;
                self.can_place_greenery_adjacent_to_owned_tiles = true;
                self.neighbours_of((*i, *j), Greenery) as i32
            }
        }
    }

    fn can_place_city_at(&self, position: (usize, usize)) -> bool {
        self.can_place_non_ocean_tile_at(position) && self.neighbours_of(position, City) == 0
    }

    fn can_place_non_ocean_tile_at(&self, position: (usize, usize)) -> bool {
        self.tiles[position.0][position.1] == Empty
            && !self.game_map.ocean_positions().contains(&position)
    }

    fn neighbours_of(&self, position: (usize, usize), tile_type: Tile) -> usize {
        Self::neighbour_positions_of(position.0, position.1)
            .into_iter()
            .filter(|position| self.tiles[position.0][position.1] == tile_type)
            .count()
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
        [
            Self::top_right_neighbour(row, column),
            Self::right_neighbour(row, column),
            Self::bottom_right_neighbour(row, column),
            Self::bottom_left_neighbour(row, column),
            Self::left_neighbour(row, column),
            Self::top_left_neighbour(row, column),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn top_left_neighbour(row: usize, column: usize) -> Option<(usize, usize)> {
        if row >= 5 {
            Some((row - 1, column))
        } else if row > 0 && column > 0 {
            Some((row - 1, column - 1))
        } else {
            None
        }
    }

    fn left_neighbour(row: usize, column: usize) -> Option<(usize, usize)> {
        if column > 0 {
            Some((row, column - 1))
        } else {
            None
        }
    }

    fn bottom_left_neighbour(row: usize, column: usize) -> Option<(usize, usize)> {
        if row <= 3 {
            Some((row + 1, column))
        } else if row <= 7 && column > 0 {
            Some((row + 1, column - 1))
        } else {
            None
        }
    }

    fn bottom_right_neighbour(row: usize, column: usize) -> Option<(usize, usize)> {
        if row <= 3 {
            Some((row + 1, column + 1))
        } else if row <= 7 && column < 12 - row {
            Some((row + 1, column))
        } else {
            None
        }
    }

    fn right_neighbour(row: usize, column: usize) -> Option<(usize, usize)> {
        if column < 8 - row.abs_diff(4) {
            Some((row, column + 1))
        } else {
            None
        }
    }

    fn top_right_neighbour(row: usize, column: usize) -> Option<(usize, usize)> {
        if row >= 5 {
            Some((row - 1, column + 1))
        } else if row > 0 && column < row + 4 {
            Some((row - 1, column))
        } else {
            None
        }
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
}
