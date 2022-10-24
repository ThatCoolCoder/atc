use pancurses::Window;

use super::utils;
use crate::command::{Command, CommandTemporality, CommandType, DirectionalCommandValue};
use crate::game::Game;
use crate::locations::{Airport, Location};
use crate::plane::{Plane, PlaneState, PlaneVisibility};

pub fn draw(window: &Window, game: &Game) {
    window.erase();
    utils::draw_standard_border(window);
    draw_stats(window, game);
    draw_plane_table(window, game);
    window.refresh();
}

fn draw_stats(window: &Window, game: &Game) {
    window.mvaddstr(
        1,
        1,
        format!("Time: {}    Safe: {}", game.ticks, game.planes_safe),
    );
}

fn draw_plane_table(window: &Window, game: &Game) {
    // Sort planes into groups
    let mut flying_planes = vec![];
    let mut waiting_planes: Vec<(&Plane, &Airport)> = vec![];
    for plane in &game.planes {
        match plane.state {
            PlaneState::Flying => flying_planes.push(plane),
            PlaneState::AtAirport(airport) => waiting_planes.push((plane, airport)),
        }
    }

    window.mvaddstr(3, 1, "pl dt  comm");

    let mut row = 4;
    // Draw flying planes
    for plane in flying_planes {
        let fuel_char = if plane.remaining_fuel <= Game::LOW_FUEL_THRESHOLD {
            '*'
        } else {
            ' '
        };
        window.mvaddstr(
            row,
            1,
            format!(
                "{plane}{fuel_char}{}  {}",
                plane.destination.to_destination_string(),
                format_plane_directional_command(plane)
            ),
        );
        row += 1;
    }

    row += 1; // newline between sections

    // Draw waiting planes
    for (plane, airport) in waiting_planes {
        window.mvaddstr(
            row,
            1,
            format!(
                "{plane} {}  Holding @ {}",
                plane.destination.to_destination_string(),
                airport.to_destination_string()
            ),
        );
    }
}

fn format_plane_directional_command(plane: &Plane) -> String {
    match plane.visibility {
        PlaneVisibility::Marked => {
            let command = plane.get_command(CommandType::Directional);
            if let Some(cmd) = command {
                if let Command::Directional(directional) = cmd {
                    let value_format = match directional.value {
                        DirectionalCommandValue::AbsoluteTurn(direction) => {
                            format!("{}", direction.to_heading())
                        }
                        DirectionalCommandValue::Circle { to_right } => {
                            let direction_char = if to_right { 'R' } else { 'L' };
                            format!("Circ {direction_char}")
                        }
                        DirectionalCommandValue::HardTurn { to_right } => {
                            let direction_char = if to_right { 'R' } else { 'L' };
                            format!("Hard {direction_char}")
                        }
                        DirectionalCommandValue::SoftTurn { to_right } => {
                            let direction_char = if to_right { 'R' } else { 'L' };
                            format!("Soft {direction_char}")
                        }
                        DirectionalCommandValue::TurnTowards(location) => {
                            format!("{}", location.to_destination_string())
                        }
                    };
                    let temporality_format = match directional.temporality {
                        CommandTemporality::Immediate => "".to_string(),
                        CommandTemporality::Delayed(beacon) => {
                            format!(" @ {}", beacon.to_destination_string())
                        }
                    };
                    format!("{value_format}{temporality_format}")
                } else {
                    panic!("We asked for a directional command but did not get one");
                }
            } else {
                "".to_string()
            }
        }
        _ => "----------".to_string(),
    }
}
