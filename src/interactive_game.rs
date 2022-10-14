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
    const FRAME_INTERVAL: i32 = 100;
    const FRAMES_PER_TICK: i32 = 30;

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

    pub fn play(&mut self) -> Result<(), LoseCondition> {
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

            if self.frame_count % Self::FRAMES_PER_TICK == 0 {
                self.game.tick()?;
            }
        }
    }

    fn buffer_to_command(&mut self) {
        let mut buffer = self.current_input_buffer.clone();
        if buffer == "" {
            self.frame_count = -1;
            self.current_input_error = "".to_string();
            return;
        }

        self.current_input_buffer = "".to_string();
        let plane_name = buffer.remove(0);
        let plane = self.game.get_plane_by_name(plane_name);
        if let None = plane {
            self.current_input_error = format!("Plane {plane_name} does not exist");
            return;
        }
    }
}
