mod command;
mod command_parser;
mod direction;
mod game;
mod graphics;
mod interactive_game;
mod levels;
mod locations;
mod plane;
mod point;

use pancurses::endwin;

fn main() {
    let level = levels::default_level::create();
    let mut interactive_game = interactive_game::InteractiveGame::from_level(&level);

    interactive_game.play();
    endwin();
}
