pub(crate) trait TilePlacementBonus {}

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    City,
    Greenery,
    Ocean,
}

#[derive(Clone)]
pub struct Board {
    tiles: [Tile; 61],
}

impl Board {
    pub fn new() -> Self {
        Self { tiles: [Tile::Empty; 61] }
    }
}
