use self::Tile::*;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tile {
    City,
    Greenery,
    Ocean,
}

impl Tile {
    pub fn is_owned(tile: Self) -> bool {
        matches!(tile, City | Greenery)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                City => 'C',
                Greenery => 'G',
                Ocean => 'O',
            }
        )
    }
}
