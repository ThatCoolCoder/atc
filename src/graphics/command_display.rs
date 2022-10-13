use pancurses::{Window, A_BOLD};

use super::utils;
use crate::game::Game;

pub fn draw(window: &Window, game: &Game) {
    utils::draw_standard_border(window);
    window.refresh();
}
