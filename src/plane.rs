use derive_more::Display;

use crate::command::*;
use crate::direction::Direction;
use crate::locations::{self, Location};
use crate::point::Point;

#[derive(Copy, Clone)]
pub enum PlaneVisibility {
    Marked,   // Fully visible
    Ignored,  // Less visible
    Unmarked, // Like ignored, but becomes marked after processing a delayed command
}

pub enum PlaneType {
    Propeller,
    Jet,
}

impl PlaneType {
    pub fn get_move_interval(&self) -> i32 {
        match self {
            PlaneType::Propeller => 2,
            PlaneType::Jet => 1,
        }
    }
}

pub enum PlaneState<'a> {
    Flying,
    AtAirport(&'a locations::Airport),
}

#[derive(Display)]
#[display(fmt = "{}{}", name, altitude)]
pub struct Plane<'a> {
    pub name: char,
    pub plane_type: PlaneType,

    pub altitude: i32,
    pub target_altitude: i32,
    pub direction: Direction,
    pub position: Point,

    pub state: PlaneState<'a>,
    pub visibility: PlaneVisibility,
    pub ticks_since_created: i32,
    pub remaining_fuel: i32,

    pub destination: &'a dyn Location,
    pub command_queue: Vec<Command<'a>>,
}

pub const PLANE_STARTING_FUEL: i32 = 30;

impl<'a> Plane<'a> {
    pub fn fly(&mut self) {
        if self.ticks_since_created % self.plane_type.get_move_interval() == 0 {
            self.parse_all_commands();
            self.update_altitude();
            self.update_position();
            self.remaining_fuel -= 1;
        }

        self.ticks_since_created += 1;
    }

    pub fn is_colliding_with(&self, other: &Plane) -> bool {
        (self.altitude - other.altitude).abs() <= 1
            && (self.position.x - other.position.x).abs() <= 1
            && (self.position.y - other.position.y).abs() <= 1
    }

    pub fn is_at_destination(&self) -> bool {
        self.position == self.destination.get_position()
    }

    fn update_altitude(&mut self) {
        if self.target_altitude < self.altitude {
            self.altitude -= 1;
        } else if self.altitude < self.target_altitude {
            self.altitude += 1;
        }
    }

    fn update_position(&mut self) {
        self.position = self.position.add(&self.direction.to_point_offset());
    }

    fn parse_all_commands(&mut self) {
        // Very convoluted method of looping over the commands, otherwise we run into borrowing issues.
        // Why must it be so hard to delegate to submethods in rust?
        let indices = &self
            .command_queue
            .iter()
            .enumerate()
            .map(|x| x.0)
            .collect::<Vec<_>>();
        for command_idx in indices {
            self.parse_command(*command_idx);
        }
    }

    fn parse_command(&mut self, command_idx: usize) -> bool {
        // Read a single command and use it to modify the plane's state if needed.
        // Returned value indicates whether the command has been fulfilled and can be removed.

        let command = &self.command_queue[command_idx];

        let do_it_now = match command.temporality {
            CommandTemporality::Immediate => true,
            CommandTemporality::Delayed(beacon) => self.position.equals(&beacon.position),
        };

        if !do_it_now {
            return false;
        }

        let should_delete: bool = match &command.value {
            CommandValue::Directional(directional_command) => match directional_command {
                DirectionalCommand::AbsoluteTurn(direction) => {
                    let mut delta = self.direction.angle_to(direction).abs();
                    let should_delete = delta <= 90;
                    delta = delta.min(90); // limit turning per turn to 90
                    let positivity = self.direction.compare_to(direction);
                    self.direction = self.direction.add_heading(positivity * delta).unwrap();
                    should_delete
                }
                DirectionalCommand::SoftTurn { to_right } => {
                    self.direction = self
                        .direction
                        .add_heading(if *to_right { 45 } else { -45 })
                        .unwrap();
                    true
                }
                DirectionalCommand::HardTurn { to_right } => {
                    self.direction = self
                        .direction
                        .add_heading(if *to_right { 45 } else { -45 })
                        .unwrap();
                    true
                }
                DirectionalCommand::TurnTowards(_) => todo!(),
                DirectionalCommand::Circle { to_right } => {
                    self.direction = self
                        .direction
                        .add_heading(if *to_right { 45 } else { -45 })
                        .unwrap();
                    false
                }
            },
            CommandValue::ChangeAltitude(altitude_command) => {
                match altitude_command {
                    ChangeAltitudeCommand::Absolute(altitude) => self.target_altitude = *altitude,
                    ChangeAltitudeCommand::Climb(amount) => {
                        self.target_altitude = self.altitude + *amount
                    }
                    ChangeAltitudeCommand::Descend(amount) => {
                        self.target_altitude = self.altitude - amount
                    }
                };
                true
            }
            CommandValue::ChangeVisibility(new_visibility) => {
                self.visibility = *new_visibility;
                true
            }
        };

        should_delete
    }
}
