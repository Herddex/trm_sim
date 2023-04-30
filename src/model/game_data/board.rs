use tile::Tile;

use crate::model::game_data::board::map::GameMap;

pub(crate) mod tile;
pub(crate) mod map;

const MAX_OCEANS: usize = 9;

#[derive(Clone)]
pub struct Board {
    placed_oceans: usize,
    tiles: [Vec<Tile>; 9],
    game_map: GameMap,
    can_place_greenery_adjacent_to_owned_tiles: bool,
}

impl Board {
    pub fn new(map: GameMap) -> Self {
        Self {
            placed_oceans: 0,
            tiles: [
                vec![Tile::Empty; 5],
                vec![Tile::Empty; 6],
                vec![Tile::Empty; 7],
                vec![Tile::Empty; 8],
                vec![Tile::Empty; 9],
                vec![Tile::Empty; 8],
                vec![Tile::Empty; 7],
                vec![Tile::Empty; 6],
                vec![Tile::Empty; 5],
            ],
            game_map: map,
            can_place_greenery_adjacent_to_owned_tiles: false,
        }
    }

    pub fn can_place(&self, tile: &Tile) -> bool {
        match tile {
            Tile::Ocean => self.placed_oceans < MAX_OCEANS,
            Tile::City | Tile::Greenery => {
                self.tiles.iter().enumerate()
                    .any(|(row, tile_row)| tile_row.iter().enumerate()
                        .any(|(column, _)| self.is_valid_placement(tile, row, column)))
            }
            _ => false
        }
    }

    pub fn place_tile(&mut self, tile: &Tile, position: (usize, usize)) -> Result<(), ()> {
        if !self.is_valid_placement(tile, position.0, position.1) { return Err(()); }

        self.tiles[position.0][position.1] = tile.clone();
        if Self::is_owned_tile(tile) {
            self.can_place_greenery_adjacent_to_owned_tiles = true
        }
        if *tile == Tile::Ocean {
            self.placed_oceans += 1
        }

        Ok(())
    }

    fn is_valid_placement(&self, tile: &Tile, row: usize, column: usize) -> bool {
        if Self::is_invalid_position(row, column) || self.tiles[row][column] != Tile::Empty {
            return false;
        }

        if self.game_map.is_ocean_position((row, column)) {
            return *tile == Tile::Ocean && self.placed_oceans < MAX_OCEANS;
        }

        match tile {
            Tile::City => !self.city_exists_around(row, column),
            Tile::Greenery => !self.can_place_greenery_adjacent_to_owned_tiles ||
                self.has_owned_tiles_around(row, column),
            _ => false
        }
    }

    fn city_exists_around(&self, row: usize, column: usize) -> bool {
        Self::neighbour_positions_of(row, column).iter()
            .any(|(row, column)| self.tiles[*row][*column] == Tile::City)
    }

    fn has_owned_tiles_around(&self, row: usize, column: usize) -> bool {
        Self::neighbour_positions_of(row, column).iter()
            .any(|(row, column)| Self::is_owned_tile(&self.tiles[*row][*column]))
    }

    fn is_owned_tile(tile: &Tile) -> bool {
        match tile {
            Tile::Greenery | Tile::City => true,
            _ => false
        }
    }

    fn neighbour_positions_of(row: usize, column: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::with_capacity(6);
        if row > 4 || row > 0 && column < row + 4 {
            neighbours.push((row - 1, column))
        }
        if row <= 4 && column < row + 4 || row > 4 && column < 12 - row {
            neighbours.push((row, column + 1));
        }
        if row < 4 || row < 8 && column < 12 - row {
            neighbours.push((row + 1, column))
        }
        if row < 8 && column > 0 {
            neighbours.push((row + 1, column - 1))
        }
        if column > 0 {
            neighbours.push((row, column - 1))
        }
        if row > 0 && column > 0 {
            neighbours.push((row - 1, column - 1))
        }
        neighbours
    }

    fn is_invalid_position(row: usize, column: usize) -> bool {
        return row > 8 || row <= 4 && column > row + 4 || row > 4 && column > 12 - row;
    }
}
