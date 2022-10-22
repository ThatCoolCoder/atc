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

    let result = interactive_game.play();
    endwin();

    match result {
        Ok(_) => (),
        Err(e) => match e {
            game::LoseCondition::PlaneCollision => println!("Planes collided with each other"),
            game::LoseCondition::PlaneIllegallyExited => println!("Plane exited illegally"),
            game::LoseCondition::PlaneHitGround => println!("Plane hit the ground"),
            game::LoseCondition::PlaneRanOutOfFuel => println!("Plane ran out of fuel"),
        },
    }
}
