use crate::command::*;
use crate::direction::Direction;
use crate::game::Game;
use crate::locations::Location;
use crate::plane::PlaneVisibility;

// Conventions of the functions within this file:
// - If their success is not guaranteed, then return Result<_, String>
// - The String is an error message designed to be displayed to the user

// First time really using lifetimes; I don't really know why it compiles but it does.

pub fn parse_command<'game, 'cmd: 'game>(
    raw_command: &str,
    game: &'game Game<'cmd>,
) -> Result<(Command<'cmd>, char), String> {
    // Second return value is name of plane

    let mut chars = raw_command.chars();
    let plane_name = chars.next().ok_or("Empty command".to_string())?;

    let command = match chars.next().ok_or(unexpected_end_of_command())? {
        't' => create_turn_command(&raw_command[2..], game),
        'c' => create_circle_command(&raw_command[2..], game),
        'a' => create_change_altitude_command(&raw_command[2..]),
        'm' => create_change_visiblity_command(PlaneVisibility::Marked),
        'u' => create_change_visiblity_command(PlaneVisibility::Unmarked),
        'i' => create_change_visiblity_command(PlaneVisibility::Ignored),
        other => Err(format!("Unknown command '{other}'"))?,
    };

    Ok((command?, plane_name))
}

fn create_change_visiblity_command<'cmd>(
    new_visibility: PlaneVisibility,
) -> Result<Command<'cmd>, String> {
    Ok(Command::ChangeVisibility(ChangeVisibilityCommand {
        new_visibility,
    }))
}

fn create_circle_command<'game, 'cmd>(
    raw_command: &str,
    game: &'game Game<'cmd>,
) -> Result<Command<'cmd>, String> {
    let (to_right, next_index) = match raw_command.chars().next() {
        Some(c) => {
            let to_right = match c {
                'l' => false,
                'r' => true,
                other => Err(format!("Unexpected direction: {other}"))?,
            };
            (to_right, 1)
        }
        None => (true, 0), // (turn to right by default)
    };
    // todo: check if delayed position is in flight path
    Ok(Command::Directional(DirectionalCommand {
        value: DirectionalCommandValue::Circle { to_right },
        temporality: determine_command_temporality(&raw_command[next_index..], game)?,
    }))
}

fn create_turn_command<'game, 'cmd: 'game>(
    raw_command: &str,
    game: &'game Game<'cmd>,
) -> Result<Command<'cmd>, String> {
    let next_char = raw_command
        .chars()
        .next()
        .ok_or(unexpected_end_of_command())?;

    let absolute_direction = Direction::from_keyboard_char(next_char);
    let (value, next_index) = match absolute_direction {
        Some(dir) => (DirectionalCommandValue::AbsoluteTurn(dir), 1),
        None => match next_char {
            'l' | '-' => (DirectionalCommandValue::SoftTurn { to_right: false }, 1),
            'r' | '+' => (DirectionalCommandValue::SoftTurn { to_right: true }, 1),
            'L' => (DirectionalCommandValue::HardTurn { to_right: false }, 1),
            'R' => (DirectionalCommandValue::HardTurn { to_right: true }, 1),
            't' => {
                let location = parse_location_subcommand(&raw_command[1..], game)?;
                (DirectionalCommandValue::TurnTowards(location), 3)
            }
            _ => Err("You stuffed up!".to_string())?,
        },
    };
    // todo: check if delayed position is in flight path
    Ok(Command::Directional(DirectionalCommand {
        value,
        temporality: determine_command_temporality(&raw_command[next_index..], game)?,
    }))
}

fn parse_location_subcommand<'game, 'cmd: 'game>(
    raw_command: &str,
    game: &'game Game<'cmd>,
) -> Result<&'cmd dyn Location, String> {
    // Parse something like 'b1' to be beacon 1
    let mut chars = raw_command.chars();
    let location_type = chars.next().ok_or(unexpected_end_of_command())?;
    let x = chars.next().ok_or(unexpected_end_of_command())?;
    let location_number = char_to_location_number(x)?;

    let location: &dyn Location = match location_type {
        'a' => {
            let airport = game
                .level
                .airports
                .iter()
                .find(|a| a.number == location_number);
            match airport {
                Some(a) => a,
                None => Err(format!("Airport {location_number} does not exist"))?,
            }
        }
        'b' => {
            let beacon = game
                .level
                .beacons
                .iter()
                .find(|b| b.number == location_number);
            match beacon {
                Some(b) => b,
                None => Err(format!("Beacon {location_number} does not exist"))?,
            }
        }
        'e' => {
            let exit = game
                .level
                .exits
                .iter()
                .find(|e| e.number == location_number);
            match exit {
                Some(e) => e,
                None => Err(format!("Exit {location_number} does not exist"))?,
            }
        }
        other => Err(format!("Unexpected location type {other}"))?,
    };
    Ok(location)
}

fn create_change_altitude_command<'game>(raw_command: &str) -> Result<Command<'game>, String> {
    let mut chars = raw_command.chars();
    let cmd = match chars.next() {
        Some(c) => match c {
            '0'..='9' => {
                let alt = char_to_altitude(c);
                Command::ChangeAltitude(ChangeAltitudeCommand::Absolute(alt?))
            }
            'c' | '+' => {
                let alt = char_to_altitude(chars.next().ok_or(unexpected_end_of_command())?)?;
                Command::ChangeAltitude(ChangeAltitudeCommand::Climb(alt))
            }
            'd' | '-' => {
                let alt = char_to_altitude(chars.next().ok_or(unexpected_end_of_command())?)?;
                Command::ChangeAltitude(ChangeAltitudeCommand::Descend(alt))
            }
            _ => Err("You messed up!".to_string())?,
        },
        None => unexpected_end_of_command_err()?,
    };

    Ok(cmd)
}

fn char_to_altitude(c: char) -> Result<i32, String> {
    char_to_int(c).ok_or(format!("Invalid altitude {c}"))
}

fn char_to_location_number(c: char) -> Result<i32, String> {
    char_to_int(c).ok_or(format!("Invalid location num {c}"))
}

fn char_to_int(c: char) -> Option<i32> {
    // Convert a char like '9' into an int.
    // returns none if char is not an int
    match c {
        // (convert to integer)
        '0'..='9' => Some(c as i32 - '0' as i32),
        _ => None,
    }
}

fn determine_command_temporality<'game, 'cmd: 'game>(
    raw_command: &str,
    game: &'game Game<'cmd>,
) -> Result<CommandTemporality<'cmd>, String> {
    let mut chars = raw_command.chars();
    match chars.next() {
        Some(_) => {
            if chars.next().is_none() {
                unexpected_end_of_command_err()?
            }
            let beacon_name = chars.next().ok_or(unexpected_end_of_command())?.to_string();
            let beacon = game
                .level
                .beacons
                .iter()
                .filter(|x| x.number.to_string() == beacon_name)
                .next();
            match beacon {
                Some(b) => Ok(CommandTemporality::Delayed(b)),
                None => Err(format!("Beacon {beacon_name} does not exist")),
            }
        }
        None => Ok(CommandTemporality::Immediate),
    }
}

fn unexpected_end_of_command() -> String {
    "Unexpected end of command".to_string()
}

fn unexpected_end_of_command_err<T>() -> Result<T, String> {
    Err(unexpected_end_of_command())
}
