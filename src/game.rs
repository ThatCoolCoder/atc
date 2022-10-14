use std::collections::HashMap;

use crate::command::*;
use crate::direction::Direction;
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
    PlaneCollision,
    PlaneIllegallyExited,
    PlaneHitGround,
    PlaneRanOutOfFuel,
}

impl<'game> Game<'game> {
    pub const MAX_PLANES: i32 = 20;
    pub const ENTRY_ALTITUDE: i32 = 7;
    pub const EXIT_ALTITUDE: i32 = 9;

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
        self.remove_safe_planes();
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

        let (direction, state, position) = if location.0.is_some() {
            // Airport case
            let location: &'game Airport = location.0.unwrap();
            (
                location.flight_direction,
                PlaneState::AtAirport(location),
                location.position,
            )
        } else {
            // Exit case
            let location = location.1.unwrap();
            (
                location.entry_direction,
                PlaneState::Flying,
                location.position,
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

            altitude: Self::ENTRY_ALTITUDE,
            target_altitude: Self::ENTRY_ALTITUDE,
            position,
            direction,

            state,
            remaining_fuel: Plane::PLANE_STARTING_FUEL,
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
        self.planes.retain(|plane| !plane.is_at_destination());
    }

    fn check_lose_conditions(&self) -> Result<(), LoseCondition> {
        let mut airport_positions = self.level.airports.iter().map(|x| x.position);

        for plane in &self.planes {
            // Check if plane has hit ground
            if plane.altitude == 0 && !airport_positions.any(|a| a.equals(&plane.position)) {
                Err(LoseCondition::PlaneHitGround)?;
            }

            // todo: check if plane illegally exited

            // Check if plane ran out of fuel
            if plane.remaining_fuel == 0 {
                Err(LoseCondition::PlaneRanOutOfFuel)?;
            }

            // Check collisions between planes
            for plane_2 in &self.planes {
                if plane.name != plane_2.name && plane.is_colliding_with(plane_2) {
                    Err(LoseCondition::PlaneCollision)?;
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

    pub fn get_plane_by_name(&self, name: char) -> Option<&Plane> {
        self.planes.iter().filter(|&p| p.name == name).next()
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
