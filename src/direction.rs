use crate::point::Point;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    pub fn from_keyboard_char(c: char) -> Option<Direction> {
        match c {
            'q' => Some(Direction::NorthWest),
            'w' => Some(Direction::North),
            'e' => Some(Direction::NorthEast),
            'a' => Some(Direction::West),
            'd' => Some(Direction::East),
            'z' => Some(Direction::SouthWest),
            'x' => Some(Direction::South),
            'c' => Some(Direction::SouthEast),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::NorthEast => '┐',
            Direction::East => '>',
            Direction::SouthEast => '┘',
            Direction::South => 'v',
            Direction::SouthWest => '└',
            Direction::West => '<',
            Direction::NorthWest => '┌',
        }
    }

    pub fn to_point_offset(&self) -> Point {
        match self {
            Direction::North => Point::new(0, -1),
            Direction::NorthEast => Point::new(1, -1),
            Direction::East => Point::new(1, 0),
            Direction::SouthEast => Point::new(1, 1),
            Direction::South => Point::new(0, 1),
            Direction::SouthWest => Point::new(-1, 1),
            Direction::West => Point::new(-1, 0),
            Direction::NorthWest => Point::new(-1, -1),
        }
    }

    pub fn to_heading(&self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::NorthEast => 45,
            Direction::East => 90,
            Direction::SouthEast => 135,
            Direction::South => 180,
            Direction::SouthWest => 225,
            Direction::West => 270,
            Direction::NorthWest => 315,
        }
    }

    pub fn from_heading(heading: i32) -> Option<Self> {
        // Convert a heading into a direction.
        // If heading does not correspond to a direction then returns None

        let modulo_heading = ((heading % 360) + 360) % 360;
        match modulo_heading {
            0 => Some(Direction::North),
            45 => Some(Direction::NorthEast),
            90 => Some(Direction::East),
            135 => Some(Direction::SouthEast),
            180 => Some(Direction::South),
            225 => Some(Direction::SouthWest),
            270 => Some(Direction::West),
            315 => Some(Direction::NorthWest),
            _ => None,
        }
    }

    pub fn angle_to(&self, other: &Self) -> i32 {
        self.angle_to_heading(other.to_heading())
    }

    pub fn angle_to_heading(&self, other: i32) -> i32 {
        let mut difference = other - self.to_heading();
        if difference < -180 {
            difference += 360;
        } else if difference > 180 {
            difference -= 360;
        }
        difference
    }

    pub fn compare_to(&self, other: &Self) -> i32 {
        // Returns -1 if other is to the left, 0 if we are equal and 1 if other is to the right
        self.compare_to_heading(other.to_heading())
    }

    pub fn compare_to_heading(&self, other: i32) -> i32 {
        // Works the same as compare_to but is slightly more performant
        // Based on https://stackoverflow.com/a/31420884/12650706

        let difference = self.angle_to_heading(other);

        // Could do this with match but it's not stable yet
        if difference > 0 {
            1
        } else if difference < 0 {
            -1
        } else {
            0
        }
    }

    pub fn add_heading(&self, heading: i32) -> Option<Direction> {
        Direction::from_heading(self.to_heading() + heading)
    }
}
