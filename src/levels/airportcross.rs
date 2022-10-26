use super::level::Level;
use crate::direction::Direction;
use crate::locations::*;
use crate::point::Point;

pub fn create() -> Level {
    Level {
        size: Point::new(19, 19),
        airports: vec![
            Airport {
                position: Point::new(7, 7),
                number: 0,
                flight_direction: Direction::SouthEast,
            },
            Airport {
                position: Point::new(11, 7),
                number: 1,
                flight_direction: Direction::SouthWest,
            },
            Airport {
                position: Point::new(7, 11),
                number: 2,
                flight_direction: Direction::NorthEast,
            },
            Airport {
                position: Point::new(11, 11),
                number: 3,
                flight_direction: Direction::NorthWest,
            },
        ],
        beacons: vec![
            // Central one
            Beacon {
                position: Point::new(9, 9),
                number: 0,
            },
            // Corners
            Beacon {
                position: Point::new(9, 1),
                number: 1,
            },
            Beacon {
                position: Point::new(1, 9),
                number: 2,
            },
            Beacon {
                position: Point::new(17, 9),
                number: 3,
            },
            Beacon {
                position: Point::new(9, 17),
                number: 4,
            },
            // Mid-edges
            Beacon {
                position: Point::new(5, 5),
                number: 5,
            },
            Beacon {
                position: Point::new(5, 13),
                number: 6,
            },
            Beacon {
                position: Point::new(13, 5),
                number: 7,
            },
            Beacon {
                position: Point::new(13, 13),
                number: 8,
            },
        ],
        exits: vec![],
        airways: vec![
            // Outer ring
            Airway {
                start: Point::new(1, 9),
                direction: Direction::NorthEast,
                length: 9,
            },
            Airway {
                start: Point::new(9, 1),
                direction: Direction::SouthEast,
                length: 9,
            },
            Airway {
                start: Point::new(17, 9),
                direction: Direction::SouthWest,
                length: 9,
            },
            Airway {
                start: Point::new(9, 17),
                direction: Direction::NorthWest,
                length: 9,
            },
            // Crossovers
            Airway {
                start: Point::new(5, 5),
                direction: Direction::SouthEast,
                length: 9,
            },
            Airway {
                start: Point::new(5, 13),
                direction: Direction::NorthEast,
                length: 9,
            },
        ],
        plane_spawn_chance: 0.1,
        move_interval: 5,
    }
}
