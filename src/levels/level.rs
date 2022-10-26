use crate::locations::*;
use crate::point::Point;

pub struct Level {
    pub description: String,
    pub plane_spawn_chance: f64, // chance of a new plane spawning on any given frame
    pub move_interval: f64,      // seconds between each move
    pub size: Point,
    pub airports: Vec<Airport>,
    pub beacons: Vec<Beacon>,
    pub exits: Vec<Exit>,
    pub airways: Vec<Airway>,
}
