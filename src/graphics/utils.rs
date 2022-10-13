use pancurses::Window;

pub fn draw_standard_border(window: &Window) {
    // Draw a border in a standardised style
    draw_border(window, '║', '║', '═', '═', '╔', '╗', '╚', '╝');
}

pub fn draw_border(
    window: &Window,
    left: char,
    right: char,
    top: char,
    bottom: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
) {
    let (height, width) = window.get_max_yx();
    if width < 2 || height < 2 {
        panic!("Game is too small!");
    }

    window.mvaddstr(0, 0, "╔");
    hline_fix(window, 0, 1, '═', width - 3);
    window.mvaddstr(0, width - 2, "╗");

    vline_fix(window, 1, 0, '║', height - 3);
    vline_fix(window, 1, width - 2, '║', height - 3);

    window.mvaddstr(height - 1, 0, "╚");
    hline_fix(window, height - 1, 1, '═', width - 3);
    window.mvaddstr(height - 1, width - 2, "╝");
}

// addch doesn't like unicode characters. pancurses used addch under the hood for hline and vline, so I've written custom versions using addstr, which does work
pub fn hline_fix(window: &Window, y: i32, x: i32, char: char, len: i32) {
    let chars = char.to_string().repeat(len as usize);
    window.mvaddstr(y, x, chars);
}

pub fn vline_fix(window: &Window, y: i32, x: i32, char: char, len: i32) {
    let mut y_pos = y + len;
    while y_pos >= y {
        window.mvaddstr(y_pos, x, char.to_string());
        y_pos -= 1;
    }
}