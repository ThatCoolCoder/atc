// Handles drawing radar, stats, planes list and credits

use pancurses::{Window, A_BOLD};

use super::utils;
use super::ColorPair;
use crate::game::Game;
use crate::plane::PlaneState;
use crate::plane::PlaneVisibility;
use crate::point::Point;

pub fn draw(window: &Window, game: &Game) {
    window.attroff(A_BOLD);
    draw_background_dots(window, game);
    draw_borders(window);
    window.attron(A_BOLD);
    draw_airways(window, game);
    draw_beacons(window, game);
    draw_exits(window, game);
    draw_airports(window, game);
    draw_planes(window, game);

    window.refresh();
}

fn draw_background_dots(window: &Window, game: &Game) {
    window.color_set(ColorPair::Markings as i16);
    let text = format!("  {}", ". ".repeat(game.level.size.x as usize - 2) + "\n")
        .repeat(game.level.size.y as usize - 1);
    window.mvaddstr(1, 0, text);
}

fn draw_borders(window: &Window) {
    window.color_set(ColorPair::Markings as i16);
    utils::draw_standard_border(window);
}

fn draw_airways(window: &Window, game: &Game) {
    window.color_set(ColorPair::Airway as i16);
    for airway in &game.level.airways {
        let mut moved_len = 0;
        let mut pos = airway.start.clone();
        let offset = airway.direction.to_point_offset();
        while moved_len < airway.length {
            let (x, y) = world_point_to_screen_position(&pos);
            window.mvaddch(y, x, '*');
            pos = pos.add(&offset);
            moved_len += 1;
        }
    }
}

fn draw_beacons(window: &Window, game: &Game) {
    window.color_set(ColorPair::Beacon as i16);
    for beacon in &game.level.beacons {
        let (x, y) = world_point_to_screen_position(&beacon.position);
        window.mvaddstr(y, x, format!("{beacon}"));
    }
}

fn draw_exits(window: &Window, game: &Game) {
    window.color_set(ColorPair::Exit as i16);
    for exit in &game.level.exits {
        let (x, y) = world_point_to_screen_position(&exit.position);
        window.mvaddstr(y, x, format!("{exit}"));
    }
}

fn draw_airports(window: &Window, game: &Game) {
    window.color_set(ColorPair::Airport as i16);
    for airport in &game.level.airports {
        let (x, y) = world_point_to_screen_position(&airport.position);
        window.mvaddstr(y, x, format!("{airport}"));
    }
}

fn draw_planes(window: &Window, game: &Game) {
    window.color_set(ColorPair::MarkedPlane as i16);
    for plane in &game.planes {
        if let PlaneState::AtAirport(_) = plane.state {
            continue;
        }
        if let PlaneVisibility::Marked = plane.visibility {
            window.attron(A_BOLD);
        } else {
            window.attroff(A_BOLD);
        }
        let (x, y) = world_point_to_screen_position(&plane.position);
        window.mvaddstr(y, x, format!("{plane}"));
    }
}

fn world_point_to_screen_position(point: &Point) -> (i32, i32) {
    (point.x * 2, point.y)
}
