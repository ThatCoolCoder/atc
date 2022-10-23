use crate::locations::*;
use crate::point::Point;

pub struct Level {
    pub size: Point,
    pub airports: Vec<Airport>,
    pub beacons: Vec<Beacon>,
    pub exits: Vec<Exit>,
    pub airways: Vec<Airway>,
    pub plane_spawn_chance: f64, // chance of a new plane spawning on any given frame
    pub move_interval: i32,      // seconds between each move
}
