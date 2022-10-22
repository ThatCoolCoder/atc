use super::level::Level;
use crate::direction::Direction;
use crate::locations::*;
use crate::point::Point;

pub fn create() -> Level {
    Level {
        size: Point::new(30, 21),
        airports: vec![
            Airport {
                position: Point::new(20, 15),
                number: 0,
                flight_direction: Direction::North,
            },
            Airport {
                position: Point::new(20, 18),
                number: 1,
                flight_direction: Direction::East,
            },
        ],
        beacons: vec![
            Beacon {
                position: Point::new(12, 7),
                number: 0,
            },
            Beacon {
                position: Point::new(12, 17),
                number: 1,
            },
        ],
        exits: vec![
            Exit {
                position: Point::new(12, 0),
                number: 0,
                entry_direction: Direction::South,
            },
            Exit {
                position: Point::new(29, 0),
                number: 1,
                entry_direction: Direction::SouthWest,
            },
            Exit {
                position: Point::new(29, 7),
                number: 2,
                entry_direction: Direction::West,
            },
            Exit {
                position: Point::new(29, 17),
                number: 3,
                entry_direction: Direction::West,
            },
            Exit {
                position: Point::new(9, 20),
                number: 4,
                entry_direction: Direction::NorthEast,
            },
            Exit {
                position: Point::new(0, 13),
                number: 5,
                entry_direction: Direction::East,
            },
            Exit {
                position: Point::new(0, 7),
                number: 6,
                entry_direction: Direction::East,
            },
            Exit {
                position: Point::new(0, 0),
                number: 7,
                entry_direction: Direction::SouthEast,
            },
        ],
        airways: vec![
            Airway {
                start: Point::new(29, 0),
                direction: Direction::SouthWest,
                length: 20,
            },
            Airway {
                start: Point::new(0, 0),
                direction: Direction::SouthEast,
                length: 7,
            },
            Airway {
                start: Point::new(0, 7),
                direction: Direction::East,
                length: 30,
            },
            Airway {
                start: Point::new(12, 0),
                direction: Direction::South,
                length: 17,
            },
            Airway {
                start: Point::new(0, 13),
                direction: Direction::East,
                length: 12,
            },
            Airway {
                start: Point::new(12, 17),
                direction: Direction::East,
                length: 18,
            },
        ],
        plane_spawn_chance: 0.05,
    }
}
