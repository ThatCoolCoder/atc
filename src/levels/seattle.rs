use indoc::indoc;

use super::level::Level;
use crate::direction::Direction;
use crate::locations::*;
use crate::point::Point;

pub fn create() -> Level {
    Level {
        description: indoc! {
            "This scenario is named Seattle not because it's based on Seattle's geography, but because of the general chaos that's present.
            Six exits, seven beacons, three airports and five potential sharp turns make for a taxing experience.
            The planes also move very quickly - twice as fast as in the Default scenario - and spawn more often too.
            You'll be thankful that with good planning most turns can be beacon-initiated. The unmark command will certainly become your friend here."
        }
        .to_string(),
        plane_spawn_chance: 0.15,
        move_interval: 2.5,
        size: Point::new(25, 21),
        airports: vec![
            Airport {
                position: Point::new(7, 11),
                number: 0,
                flight_direction: Direction::East,
            },
            Airport {
                position: Point::new(20, 9),
                number: 1,
                flight_direction: Direction::North,
            },
            Airport {
                position: Point::new(18, 16),
                number: 2,
                flight_direction: Direction::NorthWest,
            },
        ],
        beacons: vec![
            Beacon {
                position: Point::new(5, 5),
                number: 0,
            },
            Beacon {
                position: Point::new(13, 5),
                number: 1,
            },
            Beacon {
                position: Point::new(5, 8),
                number: 2,
            },
            Beacon {
                position: Point::new(16, 8),
                number: 3,
            },
            Beacon {
                position: Point::new(21, 8),
                number: 4,
            },
            Beacon {
                position: Point::new(5, 13),
                number: 5,
            },
            Beacon {
                position: Point::new(21, 13),
                number: 6,
            },
        ],
        exits: vec![
            Exit {
                position: Point::new(0, 0),
                number: 0,
                entry_direction: Direction::SouthEast,
            },
            Exit {
                position: Point::new(8, 0),
                number: 1,
                entry_direction: Direction::SouthEast,
            },
            Exit {
                position: Point::new(21, 0),
                number: 2,
                entry_direction: Direction::South,
            },
            Exit {
                position: Point::new(21, 20),
                number: 3,
                entry_direction: Direction::North,
            },
            Exit {
                position: Point::new(5, 20),
                number: 4,
                entry_direction: Direction::North,
            },
            Exit {
                position: Point::new(0, 8),
                number: 5,
                entry_direction: Direction::East,
            },
        ],
        airways: vec![
            // Comes from E0
            Airway {
                start: Point::new(0, 0),
                direction: Direction::SouthEast,
                length: 6,
            },
            // Right vertical one
            Airway {
                start: Point::new(21, 0),
                direction: Direction::South,
                length: 20,
            },
            // Comes from E1
            Airway {
                start: Point::new(8, 0),
                direction: Direction::SouthEast,
                length: 13,
            },
            // Upper full horizontal one
            Airway {
                start: Point::new(0, 8),
                direction: Direction::East,
                length: 21,
            },
            // These two come from B0
            Airway {
                start: Point::new(5, 5),
                direction: Direction::South,
                length: 15,
            },
            Airway {
                start: Point::new(5, 5),
                direction: Direction::East,
                length: 8,
            },
            // Lower partial horizontal one
            Airway {
                start: Point::new(5, 13),
                direction: Direction::East,
                length: 16,
            },
        ],
    }
}
