use winapi::um::wincon::COORD;
use super::{handle, kernel};
use shared::functions;

use winapi::um::wincon::{SetConsoleCursorPosition};


/// Set the cursor position to an coordinate (x,y).
pub fn set(x: i16, y: i16) {
    set_cursor_pos(x as i16, y as i16);
}

/// Get the current cursor x position.
pub fn xpos() -> i16 {
    let csbi = kernel::get_console_screen_buffer_info();
    csbi.dwCursorPosition.X 
}

/// Get the current cursor y position.
pub fn ypos() -> i16 {
    let csbi = kernel::get_console_screen_buffer_info();
    csbi.dwCursorPosition.Y
}

pub fn move_down(count: u16) {
    let csbi = kernel::get_console_screen_buffer_info();
    unsafe {
        let output_handle = handle::get_output_handle();
        SetConsoleCursorPosition(
            output_handle,
            COORD {
                X: csbi.dwCursorPosition.X,
                Y: csbi.dwCursorPosition.Y + count as i16,
            },
        );
    }
}

/// Set the cursor position to an coordinate (x,y).
fn set_cursor_pos(x: i16, y: i16) {
    functions::is_cursor_out_of_range(x, y);

    let output_handle = handle::get_output_handle();
    let position = COORD { X: x, Y: y };

    unsafe {
        let success = SetConsoleCursorPosition(output_handle, position);

        if success == 0 {
            panic!("Cannot set console cursor position");
        }
    }
}
