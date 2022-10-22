use crate::direction::Direction;
use crate::locations;
use crate::plane;

#[derive(PartialEq, Eq, Hash)]
pub enum CommandType {
    ChangeAltitude,
    ChangeVisibility,
    Directional,
}

pub enum Command<'cmd> {
    ChangeAltitude(ChangeAltitudeCommand),
    ChangeVisibility(ChangeVisibilityCommand),
    Directional(DirectionalCommand<'cmd>),
}

pub enum ChangeAltitudeCommand {
    Absolute(i32),
    Climb(i32),
    Descend(i32),
}

pub struct ChangeVisibilityCommand {
    pub new_visibility: plane::PlaneVisibility,
}

pub struct DirectionalCommand<'cmd> {
    pub value: DirectionalCommandValue<'cmd>,
    pub temporality: CommandTemporality<'cmd>,
}
pub enum DirectionalCommandValue<'cmd> {
    AbsoluteTurn(Direction),     // turn to a specific direction
    SoftTurn { to_right: bool }, // turn 45 deg
    HardTurn { to_right: bool }, // turn 90 deg
    TurnTowards(&'cmd dyn locations::Location),
    Circle { to_right: bool },
}
pub enum CommandTemporality<'cmd> {
    Immediate,
    Delayed(&'cmd locations::Beacon),
}
