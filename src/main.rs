mod cli;
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

use std::collections::HashMap;

fn main() {
    let mut level_map: HashMap<_, fn() -> levels::level::Level> = HashMap::new();
    // Please order the levels alphabetically
    level_map.insert("airportcross", levels::airportcross::create);
    level_map.insert("default", levels::default::create);
    level_map.insert("seattle", levels::seattle::create);
    level_map.insert("small", levels::small::create);

    let options = cli::parse_args(&level_map.keys().map(|x| x.clone()).collect());

    let level = match level_map.get(&options.level_name as &str) {
        Some(l) => l(),
        None => {
            println!(
                "Unknown level \"{}\". Run with -h option for list of levels",
                options.level_name
            );
            return;
        }
    };
    let mut interactive_game = interactive_game::InteractiveGame::from_level(&level);

    interactive_game.play();
    endwin();
}
