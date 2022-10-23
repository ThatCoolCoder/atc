use derive_more::Display;
use std::fmt;

use crate::direction::Direction;
use crate::point::Point;

// Would be lovely to make a macro to derive this but making macros looks hard
pub trait Location: fmt::Display {
    fn get_position(&self) -> Point;
    fn accessible_from_direction(&self, direction: &Direction) -> bool;
    fn can_exit_at_alt(&self, alt: i32) -> bool;
    fn to_destination_string(&self) -> String;
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
    fn can_exit_at_alt(&self, alt: i32) -> bool {
        alt == 0
    }
    fn to_destination_string(&self) -> String {
        format!("A{}", self.number)
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
    fn accessible_from_direction(&self, _direction: &Direction) -> bool {
        true
    }
    fn can_exit_at_alt(&self, _alt: i32) -> bool {
        false // (one cannot exit at a beacon at all)
    }
    fn to_destination_string(&self) -> String {
        format!("B{}", self.number)
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
    fn can_exit_at_alt(&self, alt: i32) -> bool {
        alt == 9
    }
    fn to_destination_string(&self) -> String {
        format!("E{}", self.number)
    }
}

pub struct Airway {
    pub start: Point,
    pub length: i32,
    pub direction: Direction,
}
