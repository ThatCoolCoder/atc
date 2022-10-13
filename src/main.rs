mod command;
mod command_builder;
mod command_input;
mod direction;
mod game;
mod graphics;
mod levels;
mod locations;
mod plane;
mod point;

use pancurses::{curs_set, endwin, initscr, newwin, start_color, A_BOLD};

fn main() {
    // window.keypad(true);
    // noecho();
    // loop {
    //     match window.getch() {
    //         Some(Input::Character(c)) => {
    //             window.addch(c);
    //         }
    //         Some(Input::KeyDC) => break,
    //         Some(input) => {
    //             window.addstr(&format!("{:?}", input));
    //         }
    //         None => (),
    //     }
    // }
    let level = levels::default_level::create();
    let mut game = game::Game::new(&level);

    let mut graphics_context = graphics::initialize(&game);
    loop {
        graphics::draw(&game, &mut graphics_context);
        if !game.tick().is_ok() {
            println!("Oh noes we died");
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    endwin();
}
