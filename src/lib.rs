pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, )]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn neighbor(&self, direction: Direction) -> Point {
        match direction {
            Direction::N => Point { x: self.x, y: self.y - 1 },
            Direction::E => Point { x: self.x + 1, y: self.y },
            Direction::S => Point { x: self.x, y: self.y + 1 },
            Direction::W => Point { x: self.x - 1, y: self.y },
            Direction::NE => Point{ x: self.x + 1, y: self.y - 1 },
            Direction::NW => Point { x: self.x - 1, y: self.y - 1 },
            Direction::SE => Point { x: self.x + 1, y: self.y + 1 },
            Direction::SW => Point { x: self.x - 1, y: self.y + 1 }
        }
    }
}

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

    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
            Direction::NW => Direction::NE,
            Direction::NE => Direction::SE,
            Direction::SE => Direction::SW,
            Direction::SW => Direction::NW,
        }
    }
}
