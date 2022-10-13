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

    pub fn equals(&self, other: &Point) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}
