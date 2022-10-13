use derive_more::Display;
use std::fmt;

use crate::direction::Direction;
use crate::point::Point;

// Would be lovely to make a macro to derive this but making macros looks hard
pub trait Location {
    fn get_position(&self) -> Point;
    fn accessible_from_direction(&self, direction: &Direction) -> bool;
}

pub struct Airport {
    pub position: Point,
    pub flight_direction: Direction,
    pub number: i32,
}
impl Location for Airport {
    fn get_position(&self) -> Point {
        self.position
    }
    fn accessible_from_direction(&self, direction: &Direction) -> bool {
        direction == &self.flight_direction
    }
}

// Need custom impl as
impl fmt::Display for Airport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.flight_direction.to_char(), self.number)
    }
}

#[derive(Display)]
#[display(fmt = "*{}", number)]
pub struct Beacon {
    pub position: Point,
    pub number: i32,
}
impl Location for Beacon {
    fn get_position(&self) -> Point {
        self.position
    }
    fn accessible_from_direction(&self, direction: &Direction) -> bool {
        true
    }
}

#[derive(Display)]
#[display(fmt = "{}", number)]
pub struct Exit {
    pub position: Point,
    pub number: i32,
    pub entry_direction: Direction, // Direction that planes use when entering the map from the exit
                                    // Exit direction is opposite to entry direction
}
impl Location for Exit {
    fn get_position(&self) -> Point {
        self.position
    }
    fn accessible_from_direction(&self, direction: &Direction) -> bool {
        direction == &self.entry_direction || direction == &self.entry_direction.opposite()
    }
}

pub struct Airway {
    pub start: Point,
    pub length: i32,
    pub direction: Direction,
}
