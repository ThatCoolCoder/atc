// Module . This file handles initializing the windows and managing them,

mod command_display;
mod credits_display;
mod information_display;
mod radar_display;
mod utils;

use pancurses::{
    curs_set, init_pair, initscr, newwin, start_color, Window, COLOR_BLACK, COLOR_BLUE,
    COLOR_GREEN, COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};

use crate::game::Game;

pub struct GraphicsContext {
    pub stdscr: Window,
    pub radar_window: Window,
    pub information_window: Window,
    pub command_window: Window,
    pub credits_window: Window,
}

enum ColorPair {
    Airway = 1,
    Beacon,
    Airport,
    Exit,
    MarkedPlane,
    IgnoredPlane,
    Markings,
}

const BOTTOM_ROW_HEIGHT: i32 = 5; // including border
const RIGHT_COLUMN_WIDTH: i32 = 30; // including border

pub fn initialize(game: &Game) -> GraphicsContext {
    // Setup curses and the windows required for a game
    let stdscr = initscr();
    let (height, width) = stdscr.get_max_yx();

    curs_set(0);
    start_color();
    stdscr.keypad(true);
    stdscr.nodelay(true);
    init_colors();

    let top = height - game.level.size.y - BOTTOM_ROW_HEIGHT;
    let middle_x = width / 2;

    let left_column_width = game.level.size.x * 2;
    let top_row_height = game.level.size.y;

    let radar_window = newwin(
        top_row_height,
        left_column_width,
        top,
        middle_x - left_column_width,
    );
    let information_window = newwin(top_row_height, RIGHT_COLUMN_WIDTH, top, middle_x);
    let command_window = newwin(
        BOTTOM_ROW_HEIGHT,
        left_column_width,
        height - BOTTOM_ROW_HEIGHT,
        middle_x - left_column_width,
    );
    let credits_window = newwin(
        BOTTOM_ROW_HEIGHT,
        RIGHT_COLUMN_WIDTH,
        height - BOTTOM_ROW_HEIGHT,
        middle_x,
    );

    GraphicsContext {
        stdscr,
        radar_window,
        information_window,
        command_window,
        credits_window,
    }
}

pub fn draw(game: &Game, graphics_context: &GraphicsContext, input_preview: &str) {
    radar_display::draw(&graphics_context.radar_window, game);
    information_display::draw(&graphics_context.information_window, game);
    command_display::draw(&graphics_context.command_window, game, input_preview);
    credits_display::draw(&graphics_context.credits_window, game);
}

fn init_colors() {
    init_pair(ColorPair::Airway as i16, COLOR_BLUE, COLOR_BLACK);
    init_pair(ColorPair::Beacon as i16, COLOR_RED, COLOR_BLACK);
    init_pair(ColorPair::Airport as i16, COLOR_YELLOW, COLOR_BLACK);
    init_pair(ColorPair::Exit as i16, COLOR_GREEN, COLOR_BLACK);
    init_pair(ColorPair::MarkedPlane as i16, COLOR_WHITE, COLOR_BLACK);
    init_pair(ColorPair::IgnoredPlane as i16, COLOR_WHITE, COLOR_BLACK);
    init_pair(ColorPair::Markings as i16, COLOR_WHITE, COLOR_BLACK);
}
