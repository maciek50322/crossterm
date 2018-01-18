use winapi::um::wincon::{GetConsoleScreenBufferInfo,CONSOLE_SCREEN_BUFFER_INFO};
use super::{handle, Empty};

/// Get console screen buffer info.
pub fn get_console_screen_buffer_info() -> CONSOLE_SCREEN_BUFFER_INFO {
    let output_handle = handle::get_output_handle();
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::empty();
    let success;

    if handle::is_valid_handle(&output_handle) {
        return CONSOLE_SCREEN_BUFFER_INFO::empty();
    }

    unsafe { success = GetConsoleScreenBufferInfo(output_handle, &mut csbi) }

    if success == 0 {
        panic!("Cannot get console screen buffer info");
    }

    csbi
}

/// Get the current console colors.
pub fn get_original_console_color() -> u16 {
    let console_buffer_info = get_console_screen_buffer_info();
    console_buffer_info.wAttributes as u16
}
