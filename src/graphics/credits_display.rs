use pancurses::{Window, A_BOLD};

use super::utils;
use crate::game::Game;

pub fn draw(window: &Window, _game: &Game) {
    utils::draw_standard_border(window);
    window.mvaddstr(1, 1, "Based on atc by msharov");
    window.mvaddstr(2, 1, "Implemented by ThatCoolCoder");
    window.refresh();
}