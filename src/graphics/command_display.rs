use pancurses::Window;

use super::utils;
use crate::game::Game;

pub fn draw(window: &Window, _game: &Game, command_preview: &str) {
    window.erase();
    utils::draw_standard_border(window);
    utils::wmvaddstr_multiline(window, 1, 1, command_preview);
    window.refresh();
}
