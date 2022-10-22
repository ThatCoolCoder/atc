use pancurses::Window;

use super::utils;
use crate::game::Game;

pub fn draw(window: &Window, game: &Game) {
    window.clear();
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
    window.mvaddstr(3, 1, "pl dt  comm");

    let mut row = 4;
    for plane in &game.planes {
        let fuel_char = if plane.remaining_fuel <= Game::LOW_FUEL_THRESHOLD {
            '*'
        } else {
            ' '
        };
        window.mvaddstr(
            row,
            1,
            format!(
                "{plane}{fuel_char}{}",
                plane.destination.to_destination_string()
            ),
        );
        row += 1;
    }
}
