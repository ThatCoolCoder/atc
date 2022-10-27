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
    level_map.insert("Airportcross", levels::airportcross::create);
    level_map.insert("Default", levels::default::create);
    level_map.insert("Seattle", levels::seattle::create);
    level_map.insert("Small", levels::small::create);

    let options = cli::parse_args();

    let level = match level_map.get(&options.level_name as &str) {
        Some(factory) => factory(),
        None => {
            println!(
                "Unknown level \"{}\". Run with -l option for list of levels",
                options.level_name
            );
            return;
        }
    };

    if options.show_description {
        print_description(&options.level_name, &level);
    } else if options.show_level_list {
        print_level_list(&level_map)
    } else {
        let mut interactive_game = interactive_game::InteractiveGame::from_level(&level);
        interactive_game.play();
        endwin();
    }
}

fn print_description(level_name: &str, level: &crate::levels::level::Level) {
    println!("{}", level_name);
    println!("{}\n", "-".repeat(level_name.len()));
    println!("{}", level.description);
}

fn print_level_list(level_map: &HashMap<&str, fn() -> levels::level::Level>) {
    let mut level_names: Vec<_> = level_map.keys().map(|x| x.clone()).collect();
    level_names.sort();

    println!("Available levels:");
    println!("{}", level_names.join("\n"));
}
