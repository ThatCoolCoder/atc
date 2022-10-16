use crate::command::*;
use crate::game::Game;
use crate::plane::PlaneVisibility;

// Conventions of the functions within this file:
// - If their success is not guaranteed, then return Result<_, String>
// - The String is an error message designed to be displayed to the user

pub fn parse_command<'a>(raw_command: &str, game: &'a Game) -> Result<(Command<'a>, char), String> {
    // Second return value is name of plane

    let mut chars = raw_command.chars();
    let plane_name = chars.next().ok_or("Empty command".to_string())?;

    let command = match chars.next().ok_or(unexpected_end_of_command())? {
        // 't' => create_turn_command(chars),
        'c' => create_circle_command(&raw_command[2..], game),
        'a' => create_change_altitude_command(&raw_command[2..]),
        'm' => create_change_visiblity_command(PlaneVisibility::Marked),
        'u' => create_change_visiblity_command(PlaneVisibility::Unmarked),
        'i' => create_change_visiblity_command(PlaneVisibility::Ignored),
        other => Err(format!("Unknown command '{other}'"))?,
    };

    Ok((command?, plane_name))
}

fn create_change_visiblity_command<'a>(
    new_visibility: PlaneVisibility,
) -> Result<Command<'a>, String> {
    Ok(Command::ChangeVisibility(ChangeVisibilityCommand {
        new_visibility,
    }))
}

fn create_circle_command<'a>(raw_command: &str, game: &'a Game) -> Result<Command<'a>, String> {
    let to_right = match raw_command.chars().next() {
        Some(c) => match c {
            'l' => false,
            'r' => true,
            other => Err(format!("Unexpected direction: {other}"))?,
        },
        None => unexpected_end_of_command_err()?,
    };
    // todo: check if delayed position is in flight path
    Ok(Command::Directional(DirectionalCommand {
        value: DirectionalCommandValue::Circle { to_right },
        temporality: determine_command_temporality(&raw_command[1..], game)?,
    }))
}

fn create_turn_command<'a>(raw_command: &str) -> Result<Command<'a>, String> {
    // todo: check if delayed position is in flight path
    Err("You stuffed up".to_string())
}

fn create_change_altitude_command<'a>(raw_command: &str) -> Result<Command<'a>, String> {
    Err("You stuffed up".to_string())
}

fn determine_command_temporality<'a>(
    raw_command: &str,
    game: &'a Game,
) -> Result<CommandTemporality<'a>, String> {
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
