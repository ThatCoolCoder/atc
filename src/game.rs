use std::collections::HashMap;
use std::fmt;

use crate::levels::level::Level;
use crate::locations::*;
use crate::plane::*;

use rand::Rng;

pub struct Game<'game> {
    pub planes: Vec<Plane<'game>>,
    pub level: &'game Level,
    pub ticks: i32,
    pub planes_safe: i32,
}

pub enum LoseCondition {
    PlaneCollision { plane_a: char, plane_b: char },
    PlaneIllegallyExited { plane: char },
    PlaneHitGround { plane: char },
    PlaneRanOutOfFuel { plane: char },
}

impl fmt::Display for LoseCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoseCondition::PlaneCollision { plane_a, plane_b } => {
                write!(f, "Plane {plane_a} hit plane {plane_b}")
            }
            LoseCondition::PlaneIllegallyExited { plane } => {
                write!(f, "Plane {plane} exited illegally")
            }
            LoseCondition::PlaneHitGround { plane } => {
                write!(f, "Plane {plane} hit the ground")
            }
            LoseCondition::PlaneRanOutOfFuel { plane } => {
                write!(f, "Plane {plane} ran out of fuel")
            }
        }
    }
}

impl<'game> Game<'game> {
    pub const MAX_PLANES: i32 = 20;
    pub const ENTRY_ALTITUDE: i32 = 7;
    pub const AIRPORT_ENTRY_ALTITUDE: i32 = 0;
    pub const LOW_FUEL_THRESHOLD: i32 = 15;

    pub fn new(level: &'game Level) -> Self {
        let mut g = Game {
            planes: vec![],
            level: level,
            ticks: 0,
            planes_safe: 0,
        };
        g.create_new_plane();
        g
    }

    pub fn tick(&mut self) -> Result<(), LoseCondition> {
        self.ticks += 1;

        self.maybe_create_new_plane();
        self.move_planes();
        self.remove_safe_planes(); // remove safe before checking lose so that EG planes that just landed don't count as crashed
        self.check_lose_conditions()?;

        Ok(())
    }

    fn maybe_create_new_plane(&mut self) {
        if self.planes.len() < Self::MAX_PLANES as usize {
            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() <= self.level.plane_spawn_chance {
                self.create_new_plane();
            }
        }
    }

    fn create_new_plane(&mut self) {
        // Randomly spawn a new plane
        let mut rng = rand::thread_rng();

        let location = self.random_airport_or_exit();

        let (direction, state, position, alt) = if location.0.is_some() {
            // Airport case
            let location: &'game Airport = location.0.unwrap();
            (
                location.flight_direction,
                PlaneState::AtAirport(location),
                location.position,
                Self::AIRPORT_ENTRY_ALTITUDE,
            )
        } else {
            // Exit case
            let location = location.1.unwrap();
            (
                location.entry_direction,
                PlaneState::Flying,
                location.position,
                Self::ENTRY_ALTITUDE,
            )
        };

        let destination_tuple = self.random_airport_or_exit();
        let destination: &dyn Location = if destination_tuple.0.is_some() {
            destination_tuple.0.unwrap()
        } else {
            destination_tuple.1.unwrap()
        };

        // Create plane
        self.planes.push(Plane {
            name: self.next_free_plane_name(),
            plane_type: if rng.gen::<f64>() < 0.5 {
                PlaneType::Jet
            } else {
                PlaneType::Propeller
            },

            altitude: alt,
            target_altitude: alt,
            position,
            direction,

            state,
            remaining_fuel: self.level.size.x + self.level.size.y,
            visibility: PlaneVisibility::Marked,
            ticks_since_created: 0,
            destination,
            command_queue: vec![],
            command_map: HashMap::new(),
        })
    }

    fn move_planes(&mut self) {
        for plane in &mut self.planes {
            plane.fly();
        }
    }

    fn remove_safe_planes(&mut self) {
        let old_len = self.planes.len();
        self.planes.retain(|plane| !plane.is_at_destination());
        self.planes_safe += (old_len - self.planes.len()) as i32;
    }

    fn check_lose_conditions(&self) -> Result<(), LoseCondition> {
        for plane in &self.planes {
            // Check if plane has hit ground
            if plane.altitude == 0 && !plane.is_at_airport() {
                Err(LoseCondition::PlaneHitGround { plane: plane.name })?;
            }

            let is_out_of_bounds = plane.position.x <= 0
                || plane.position.y <= 0
                || plane.position.x >= self.level.size.x - 1
                || plane.position.y >= self.level.size.y - 1;
            if is_out_of_bounds && plane.ticks_since_created > 1 {
                // (don't kill planes that have just entered because that makes no sense)
                Err(LoseCondition::PlaneIllegallyExited { plane: plane.name })?;
            }

            // Check if plane ran out of fuel
            if plane.remaining_fuel == 0 {
                Err(LoseCondition::PlaneRanOutOfFuel { plane: plane.name })?;
            }

            // Check collisions between planes
            for plane_2 in &self.planes {
                if !plane.is_at_airport()
                    && !plane_2.is_at_airport()
                    && plane.name != plane_2.name
                    && plane.is_colliding_with(plane_2)
                {
                    Err(LoseCondition::PlaneCollision {
                        plane_a: plane.name,
                        plane_b: plane_2.name,
                    })?;
                }
            }
        }
        Ok(())
    }

    fn next_free_plane_name(&self) -> char {
        let existing_plane_names: Vec<_> = self.planes.iter().map(|x| x.name).collect();
        for ch in b'a'..=b'z' {
            if !existing_plane_names.contains(&(ch as char)) {
                return ch as char;
            }
        }
        panic!("Could not find a spare char for plane names (this should not be possible)");
    }

    #[allow(unused)]
    pub fn get_plane_by_name(&self, name: char) -> Option<&Plane> {
        self.planes.iter().filter(|&p| p.name == name).next()
    }

    pub fn get_plane_by_name_mut(&mut self, name: char) -> Option<&mut Plane<'game>> {
        self.planes.iter_mut().filter(|p| p.name == name).next()
    }

    fn random_airport_or_exit(&self) -> (Option<&'game Airport>, Option<&'game Exit>) {
        let mut rng = rand::thread_rng();
        let num_possibilities = self.level.exits.len() + self.level.airports.len() - 1;
        let spawn_point_idx: usize = rng.gen_range(0..num_possibilities);

        if spawn_point_idx < self.level.exits.len() {
            (None, Some(&self.level.exits[spawn_point_idx]))
        } else {
            (
                Some(&self.level.airports[spawn_point_idx - self.level.exits.len()]),
                None,
            )
        }
    }
}
