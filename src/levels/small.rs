use indoc::indoc;

use super::level::Level;
use crate::direction::Direction;
use crate::locations::*;
use crate::point::Point;

pub fn create() -> Level {
    Level {
        description: indoc! {
            "It's a small level, but that doesn't make it easy. Three airports facing inwards near the borders require much sheperding of planes.
            There are also two potential tight corners that have to be manually initiated, as beacon-initiated turns would be too wide."
        }
        .to_string(),
        plane_spawn_chance: 0.125,
        move_interval: 4.,
        size: Point::new(15, 15),
        airports: vec![
            Airport {
                position: Point::new(6, 3),
                number: 0,
                flight_direction: Direction::South,
            },
            Airport {
                position: Point::new(11, 10),
                number: 1,
                flight_direction: Direction::West,
            },
            Airport {
                position: Point::new(4, 12),
                number: 2,
                flight_direction: Direction::North,
            },
        ],
        beacons: vec![
            Beacon {
                position: Point::new(6, 6),
                number: 0,
            },
            Beacon {
                position: Point::new(6, 10),
                number: 1,
            },
        ],
        exits: vec![
            Exit {
                position: Point::new(0, 0),
                number: 0,
                entry_direction: Direction::SouthEast,
            },
            // Exit {
            //     position: Point::new(14, 2),
            //     number: 1,
            //     entry_direction: Direction::SouthWest,
            // },
            // Exit {
            //     position: Point::new(14, 6),
            //     number: 2,
            //     entry_direction: Direction::West,
            // },
            // Exit {
            //     position: Point::new(6, 14),
            //     number: 3,
            //     entry_direction: Direction::North,
            // },
            // Exit {
            //     position: Point::new(0, 10),
            //     number: 4,
            //     entry_direction: Direction::East,
            // },
        ],
        airways: vec![
            Airway {
                start: Point::new(0, 0),
                direction: Direction::SouthEast,
                length: 6,
            },
            Airway {
                start: Point::new(14, 2),
                direction: Direction::SouthWest,
                length: 9,
            },
            Airway {
                start: Point::new(0, 10),
                direction: Direction::East,
                length: 7,
            },
            Airway {
                start: Point::new(6, 14),
                direction: Direction::North,
                length: 8,
            },
            Airway {
                start: Point::new(6, 6),
                direction: Direction::East,
                length: 9,
            },
        ],
    }
}
