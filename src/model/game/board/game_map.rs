use crate::model::game::board::BoardPosition;
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref THARSIS: GameMap = GameMap::new(HashSet::from([
        (0, 1),
        (0, 3),
        (0, 4),
        (1, 5),
        (3, 7),
        (4, 3),
        (4, 4),
        (4, 5),
        (5, 5),
        (5, 6),
        (5, 7),
        (8, 4)
    ]));
}

#[derive(Clone)]
pub struct GameMap {
    ocean_positions: HashSet<(usize, usize)>,
}

impl GameMap {
    pub fn is_ocean_position(&self, position: BoardPosition) -> bool {
        self.ocean_positions.contains(&position)
    }

    pub fn ocean_positions(&self) -> &HashSet<(usize, usize)> {
        &self.ocean_positions
    }

    fn new(ocean_positions: HashSet<(usize, usize)>) -> Self {
        Self { ocean_positions }
    }
}
