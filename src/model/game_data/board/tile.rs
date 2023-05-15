use std::fmt::{Display, Formatter};
use self::Tile::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tile {
    Empty,
    City,
    Greenery,
    Ocean,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Empty => '*',
            City => 'C',
            Greenery => 'G',
            Ocean => 'O'
        })
    }
}
