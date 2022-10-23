use crate::command_parser;
use crate::game::{Game, LoseCondition};
use crate::graphics::{self, GraphicsContext};
use crate::levels::level::Level;

use pancurses::Input;

pub struct InteractiveGame<'game> {
    // Extension of the basic game that provides support for drawing to the screen and getting input from keyboard
    current_input_buffer: String,
    current_input_error: String,
    pub game: Game<'game>,
    graphics_context: GraphicsContext,
    frame_count: i32,
}

impl<'game> InteractiveGame<'game> {
    const FRAME_INTERVAL: i32 = 30;

    pub fn from_level(level: &'game Level) -> Self {
        Self::from_game(Game::new(level))
    }

    pub fn from_game(game: Game<'game>) -> Self {
        let graphics_context = graphics::initialize(&game);
        InteractiveGame {
            current_input_buffer: "".to_string(),
            current_input_error: "".to_string(),
            game,
            graphics_context,
            frame_count: 0,
        }
    }

    pub fn play(&mut self) {
        let result: Result<(), LoseCondition>;
        let frames_per_tick = self.game.level.move_interval / Self::FRAME_INTERVAL;
        loop {
            match self.graphics_context.stdscr.getch() {
                Some(input) => match input {
                    Input::Character(c) => {
                        if c == '\n' {
                            self.buffer_to_command();
                        } else {
                            self.current_input_buffer.push(c)
                        }
                    }
                    Input::KeyBackspace => {
                        self.current_input_buffer.pop();
                    }
                    _ => (),
                },
                None => (),
            }

            let input_preview = if self.current_input_buffer == "" {
                &self.current_input_error
            } else {
                &self.current_input_buffer
            };

            graphics::draw(&self.game, &self.graphics_context, input_preview);

            std::thread::sleep(std::time::Duration::from_millis(
                Self::FRAME_INTERVAL as u64,
            ));
            self.frame_count += 1;

            if self.frame_count % frames_per_tick == 0 {
                match self.game.tick() {
                    Ok(()) => (),
                    Err(e) => {
                        result = Err(e);
                        break;
                    }
                }
            }
        }

        let result_text = match result {
            Ok(_) => "somehow we got an ok result here, how?".to_string(),
            Err(e) => e.to_string(),
        } + ". Press space to exit";
        graphics::draw(&self.game, &self.graphics_context, &result_text);

        // Wait until enter pressed
        loop {
            if let Some(input) = self.graphics_context.stdscr.getch() {
                if let Input::Character(c) = input {
                    if c == ' ' {
                        break;
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(
                Self::FRAME_INTERVAL as u64,
            ));
        }
    }

    fn buffer_to_command(&mut self) {
        let buffer = self.current_input_buffer.clone();
        self.current_input_buffer = "".to_string();

        if buffer == "" {
            self.frame_count = -1; // todo: make a better way of resetting the frame counter
            self.current_input_error = "".to_string();
            return;
        }

        match command_parser::parse_command(&buffer, &self.game) {
            Ok((command, plane_name)) => {
                self.current_input_error = "".to_string();
                let plane = self.game.get_plane_by_name_mut(plane_name);
                match plane {
                    Some(p) => p.add_command(command),
                    None => self.current_input_error = format!("Plane {plane_name} does not exist"),
                }
            }
            Err(error) => self.current_input_error = error,
        }
    }
}
