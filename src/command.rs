use crate::direction::Direction;
use crate::locations;
use crate::plane;

#[derive(PartialEq, Eq, Hash)]
pub enum CommandType {
    ChangeAltitude,
    ChangeVisibility,
    Directional,
}

pub enum Command<'a> {
    ChangeAltitude(ChangeAltitudeCommand),
    ChangeVisibility(ChangeVisibilityCommand),
    Directional(DirectionalCommand<'a>),
}

pub enum ChangeAltitudeCommand {
    Absolute(i32),
    Climb(i32),
    Descend(i32),
}

pub struct ChangeVisibilityCommand {
    pub new_visibility: plane::PlaneVisibility,
}

pub struct DirectionalCommand<'a> {
    pub value: DirectionalCommandValue<'a>,
    pub temporality: CommandTemporality<'a>,
}
pub enum DirectionalCommandValue<'a> {
    AbsoluteTurn(Direction),     // turn to a specific direction
    SoftTurn { to_right: bool }, // turn 45 deg
    HardTurn { to_right: bool }, // turn 90 deg
    TurnTowards(&'a dyn locations::Location),
    Circle { to_right: bool },
}
pub enum CommandTemporality<'a> {
    Immediate,
    Delayed(&'a locations::Beacon),
}
