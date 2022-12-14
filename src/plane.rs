use std::collections::HashMap;
use std::fmt;

use crate::command::*;
use crate::direction::Direction;
use crate::locations::{self, Location};
use crate::point::Point;

#[derive(Copy, Clone, PartialEq)]
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
    pub command_map: HashMap<CommandType, Command<'a>>,
}

impl<'a> fmt::Display for Plane<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let capitalised_name = match self.plane_type {
            PlaneType::Propeller => self.name.to_ascii_uppercase(),
            PlaneType::Jet => self.name.to_ascii_lowercase(),
        };
        write!(f, "{}{}", capitalised_name, self.altitude)
    }
}

impl<'a> Plane<'a> {
    pub fn fly(&mut self) {
        if self.ticks_since_created % self.plane_type.get_move_interval() == 0 {
            self.parse_all_commands();
            self.update_altitude();
            if !self.is_at_airport() {
                self.update_position();
                self.remaining_fuel -= 1;
            }
        }

        self.ticks_since_created += 1;
    }

    pub fn add_command(&mut self, command: Command<'a>) {
        let command_type = match command {
            Command::ChangeAltitude(_) => CommandType::ChangeAltitude,
            Command::Directional(_) => CommandType::Directional,
            Command::ChangeVisibility(_) => CommandType::ChangeVisibility,
        };
        self.command_map.insert(command_type, command);
    }

    pub fn get_command(&self, command_type: CommandType) -> Option<&Command> {
        self.command_map.get(&command_type)
    }

    pub fn is_colliding_with(&self, other: &Plane) -> bool {
        (self.altitude - other.altitude).abs() <= 1
            && (self.position.x - other.position.x).abs() <= 1
            && (self.position.y - other.position.y).abs() <= 1
    }

    pub fn is_at_destination(&self) -> bool {
        self.position.equals(&self.destination.get_position())
            && self.destination.accessible_from_direction(&self.direction)
            && self.destination.can_exit_at_alt(self.altitude)
    }

    pub fn is_at_airport(&self) -> bool {
        match self.state {
            PlaneState::AtAirport(_) => true,
            _ => false,
        }
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

        if self.parse_command(CommandType::Directional) {
            self.command_map.remove(&CommandType::Directional);
        }
        if self.parse_command(CommandType::ChangeVisibility) {
            self.command_map.remove(&CommandType::ChangeVisibility);
        }
        if self.parse_command(CommandType::ChangeAltitude) {
            self.command_map.remove(&CommandType::ChangeAltitude);
        }
    }

    fn parse_command(&mut self, command_type: CommandType) -> bool {
        let mut command = match self.command_map.get_mut(&command_type) {
            Some(v) => v,
            None => return false,
        };

        let should_delete: bool = match &mut command {
            Command::Directional(directional_command) => {
                // Check if we should do the command now
                let do_it_now = match directional_command.temporality {
                    CommandTemporality::Immediate => true,
                    CommandTemporality::Delayed(l) => l.position.equals(&self.position),
                };
                if !do_it_now {
                    return false;
                }

                // Now that command has started to be processed, it should continue
                directional_command.temporality = CommandTemporality::Immediate;

                // Planes at airport can't change direction
                if let PlaneState::AtAirport(_) = self.state {
                    return true;
                }

                // Modify unmarked state
                if self.visibility == PlaneVisibility::Unmarked {
                    self.visibility = PlaneVisibility::Marked;
                }

                // Actually run the command
                match &directional_command.value {
                    DirectionalCommandValue::AbsoluteTurn(direction) => {
                        self.direction = Direction::limited_rotate(&self.direction, &direction);
                        let should_delete = self.direction == *direction;
                        should_delete
                    }
                    DirectionalCommandValue::SoftTurn { to_right } => {
                        self.direction = self
                            .direction
                            .add_heading(if *to_right { 45 } else { -45 })
                            .unwrap();
                        true
                    }
                    DirectionalCommandValue::HardTurn { to_right } => {
                        self.direction = self
                            .direction
                            .add_heading(if *to_right { 45 } else { -45 })
                            .unwrap();
                        true
                    }
                    DirectionalCommandValue::TurnTowards(location) => {
                        let position_delta = location.get_position().sub(&self.position);
                        let target_heading = position_delta.heading().to_degrees();
                        let target_heading = ((target_heading / 45.).round() * 45.) as i32;
                        let target_direction = Direction::from_heading(target_heading).unwrap();
                        self.direction =
                            Direction::limited_rotate(&self.direction, &target_direction);
                        let should_delete = self.direction == target_direction;

                        should_delete
                    }
                    DirectionalCommandValue::Circle { to_right } => {
                        self.direction = self
                            .direction
                            .add_heading(if *to_right { 90 } else { -90 })
                            .unwrap();
                        false
                    }
                }
            }
            Command::ChangeAltitude(altitude_command) => {
                self.state = PlaneState::Flying;
                match altitude_command {
                    ChangeAltitudeCommand::Absolute(altitude) => self.target_altitude = *altitude,
                    ChangeAltitudeCommand::Climb(amount) => {
                        self.target_altitude = self.altitude + *amount
                    }
                    ChangeAltitudeCommand::Descend(amount) => {
                        self.target_altitude = self.altitude - *amount
                    }
                };
                true
            }
            Command::ChangeVisibility(visibility_command) => {
                self.visibility = visibility_command.new_visibility;
                true
            }
        };

        should_delete
    }
}
