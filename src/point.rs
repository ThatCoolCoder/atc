// Minimal 2d vector implementation that is added to as needed

use std::f64::consts::{PI, TAU};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[allow(unused)]
impl Point {
    pub fn zero() -> Point {
        Point::new(0, 0)
    }

    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn add(&self, other: &Point) -> Point {
        return Point::new(self.x + other.x, self.y + other.y);
    }

    pub fn sub(&self, other: &Point) -> Point {
        return Point::new(self.x - other.x, self.y - other.y);
    }

    pub fn equals(&self, other: &Point) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    pub fn heading(&self) -> f64 {
        // (zero heading is (0, 1))
        ((self.y as f64).atan2(self.x as f64) + PI / 2.).rem_euclid(TAU)
    }
}
