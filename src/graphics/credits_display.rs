use pancurses::Window;

use super::utils;
use crate::game::Game;

pub fn draw(window: &Window, _game: &Game) {
    window.erase();
    utils::draw_standard_border(window);
    window.mvaddstr(1, 1, "Implemented by ThatCoolCoder");
    window.mvaddstr(2, 1, "Based on msharov's version,");
    window.mvaddstr(3, 1, "which is based on Ed James's");
    window.refresh();
}
