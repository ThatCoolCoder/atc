use super::level::Level;
use crate::direction::Direction;
use crate::locations::*;
use crate::point::Point;

pub fn create() -> Level {
    Level {
        size: Point::new(17, 17),
        airports: vec![
            Airport {
                position: Point::new(4, 4),
                number: 0,
                flight_direction: Direction::SouthEast,
            },
            Airport {
                position: Point::new(10, 4),
                number: 1,
                flight_direction: Direction::SouthWest,
            },
            Airport {
                position: Point::new(4, 10),
                number: 2,
                flight_direction: Direction::NorthEast,
            },
            Airport {
                position: Point::new(10, 10),
                number: 3,
                flight_direction: Direction::NorthWest,
            },
        ],
        beacons: vec![
            Beacon {
                position: Point::new(9, 1),
                number: 0,
            },
            Beacon {
                position: Point::new(1, 9),
                number: 1,
            },
            Beacon {
                position: Point::new(17, 9),
                number: 2,
            },
            Beacon {
                position: Point::new(9, 17),
                number: 3,
            },
        ],
        exits: vec![],
        airways: vec![
            Airway {
                start: Point::new(1, 1),
                direction: Direction::East,
                length: 13,
            },
            Airway {
                start: Point::new(1, 13),
                direction: Direction::East,
                length: 13,
            },
            Airway {
                start: Point::new(13, 1),
                direction: Direction::South,
                length: 13,
            },
            Airway {
                start: Point::new(1, 1),
                direction: Direction::South,
                length: 13,
            },
            Airway {
                start: Point::new(1, 1),
                direction: Direction::SouthEast,
                length: 13,
            },
            Airway {
                start: Point::new(13, 1),
                direction: Direction::SouthWest,
                length: 13,
            },
        ],
        plane_spawn_chance: 0.1,
        move_interval: 5,
    }
}
