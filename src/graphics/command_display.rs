use pancurses::{Window, A_BOLD};

use super::utils;
use crate::game::Game;

pub fn draw(window: &Window, game: &Game, command_preview: &str) {
    window.clear();
    utils::draw_standard_border(window);
    window.mvaddstr(1, 1, command_preview);
    window.refresh();
}
