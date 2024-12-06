pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

pub const CARDINALS: [Direction; 4] = [Direction::N, Direction::E, Direction::S, Direction::W];

pub const ORDINALS: [Direction; 4] = [Direction::NW, Direction::NE, Direction::SE, Direction::SW];

pub const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::E,
    Direction::S,
    Direction::W,
    Direction::NW,
    Direction::NE,
    Direction::SE,
    Direction::SW,
];

impl Direction {
    pub fn invert(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::NW => Direction::SE,
            Direction::NE => Direction::SW,
            Direction::SE => Direction::NW,
            Direction::SW => Direction::NE,
        }
    }
}
