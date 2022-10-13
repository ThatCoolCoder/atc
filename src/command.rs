use crate::direction::Direction;
use crate::locations;
use crate::plane;

pub enum CommandValue<'a> {
    Directional(DirectionalCommand<'a>),
    ChangeAltitude(ChangeAltitudeCommand),
    ChangeVisibility(plane::PlaneVisibility),
}

pub enum CommandTemporality<'a> {
    Immediate,
    Delayed(&'a locations::Beacon),
}

pub struct Command<'a> {
    pub value: CommandValue<'a>,
    pub temporality: CommandTemporality<'a>,
}

pub enum DirectionalCommand<'a> {
    AbsoluteTurn(Direction),     // turn to a specific direction
    SoftTurn { to_right: bool }, // turn 45 deg
    HardTurn { to_right: bool }, // turn 90 deg
    TurnTowards(&'a dyn locations::Location),
    Circle { to_right: bool },
}

pub enum ChangeAltitudeCommand {
    Absolute(i32),
    Climb(i32),
    Descend(i32),
}
