use std::collections::HashSet;

#[derive(Clone)]
pub struct GameMap {
    ocean_positions: HashSet<(usize, usize)>
}

impl GameMap {
    pub fn new(ocean_positions: HashSet<(usize, usize)>) -> Self {
        Self { ocean_positions }
    }
    pub fn is_ocean_position(&self, position: (usize, usize)) -> bool {
        self.ocean_positions.contains(&position)
    }
}