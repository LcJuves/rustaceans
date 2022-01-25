//! https://docs.microsoft.com/zh-cn/windows/console/clearing-the-screen

#![cfg(windows)]

use crate::vtesc_seq;

use core::ffi::c_void;

use std::env::var;
use std::ptr::{null, null_mut};
use std::sync::Once;

use lazy_static::lazy_static;
use windows::core::{Error, Result};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Console::GetConsoleMode;
use windows::Win32::System::Console::GetConsoleScreenBufferInfo;
use windows::Win32::System::Console::GetStdHandle;
use windows::Win32::System::Console::ScrollConsoleScreenBufferW;
use windows::Win32::System::Console::SetConsoleCursorPosition;
use windows::Win32::System::Console::SetConsoleMode;
use windows::Win32::System::Console::SetConsoleTextAttribute;
use windows::Win32::System::Console::WriteConsoleW;
use windows::Win32::System::Console::CHAR_INFO;
use windows::Win32::System::Console::CONSOLE_MODE;
use windows::Win32::System::Console::CONSOLE_SCREEN_BUFFER_INFO;
use windows::Win32::System::Console::COORD;
use windows::Win32::System::Console::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
use windows::Win32::System::Console::SMALL_RECT;
use windows::Win32::System::Console::STD_OUTPUT_HANDLE;

lazy_static! {
    static ref DEFAULT_WATTRIBUTES: u16 = {
        let mut buf_info = CONSOLE_SCREEN_BUFFER_INFO::default();
        let stdout_handle = *STDOUT_HANDLE;
        unsafe {
            GetConsoleScreenBufferInfo(
                stdout_handle,
                &mut buf_info as *mut CONSOLE_SCREEN_BUFFER_INFO,
            )
            .ok()
            .unwrap_or(());
        }
        buf_info.wAttributes
    };
    static ref IS_XTERM: bool = {
        if let Ok(term) = var("TERM") {
            return term == "xterm";
        }
        false
    };
    pub(crate) static ref STDOUT_HANDLE: HANDLE = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
}

const ONCE_INIT: Once = Once::new();

pub(crate) fn get_stdout_handle() -> HANDLE {
    let stdout_handle = *STDOUT_HANDLE;
    ONCE_INIT.call_once(|| {
        let _ = *DEFAULT_WATTRIBUTES;
    });
    stdout_handle
}

fn set_con_text_attr(wattributes: u16) {
    unsafe {
        let stdout_handle = get_stdout_handle();
        SetConsoleTextAttribute(stdout_handle, wattributes).ok().unwrap_or(());
    }
}

fn write_conw(ansi_str: &str) -> Result<()> {
    unsafe {
        let stdout_handle = get_stdout_handle();

        let mut mode: CONSOLE_MODE = 0;
        if !GetConsoleMode(stdout_handle, &mut mode as *mut CONSOLE_MODE).as_bool() {
            return Err(Error::from_win32());
        }

        let original_mode = mode;
        mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        if !SetConsoleMode(stdout_handle, mode).as_bool() {
            return Err(Error::from_win32());
        }

        let mut sequence = ansi_str.encode_utf16().collect::<Vec<u16>>();

        if !WriteConsoleW(
            stdout_handle,
            sequence.as_mut_ptr() as *const c_void,
            sequence.len() as u32,
            null_mut() as *mut u32,
            null_mut() as *mut c_void,
        )
        .as_bool()
        {
            // If we fail, try to restore the mode on the way out.
            SetConsoleMode(stdout_handle, original_mode);
            return Err(Error::from_win32());
        }

        // Restore the mode on the way out to be nice to other command-line applications.
        SetConsoleMode(stdout_handle, original_mode);
    }
    Ok(())
}

pub(crate) fn reset() {
    if *IS_XTERM {
        print!("{}", vtesc_seq::NONE);
    } else {
        let wattributes = *DEFAULT_WATTRIBUTES;
        set_con_text_attr(wattributes);
    }
}

pub(crate) fn set_red() {
    if let Err(_) = write_conw(vtesc_seq::RED) {
        if *IS_XTERM {
            print!("{}", vtesc_seq::RED);
        } else {
            set_con_text_attr(0x4);
        }
    }
}

pub(crate) fn set_green() {
    if let Err(_) = write_conw(vtesc_seq::GREEN) {
        if *IS_XTERM {
            print!("{}", vtesc_seq::GREEN);
        } else {
            set_con_text_attr(0x2);
        }
    }
}

pub(crate) fn set_blue() {
    if let Err(_) = write_conw(vtesc_seq::BLUE) {
        if *IS_XTERM {
            print!("{}", vtesc_seq::BLUE);
        } else {
            set_con_text_attr(0x1);
        }
    }
}

pub(crate) fn set_white() {
    if let Err(_) = write_conw(vtesc_seq::WHITE) {
        if *IS_XTERM {
            print!("{}", vtesc_seq::WHITE);
        } else {
            set_con_text_attr(0x7);
        }
    }
}

pub(crate) fn set_high_light() {
    if let Err(_) = write_conw(vtesc_seq::HIGH_LIGHT) {
        if *IS_XTERM {
            print!("{}", vtesc_seq::HIGH_LIGHT);
        } else {
            todo!();
        }
    }
}

pub(crate) fn set_under_line() {
    if let Err(_) = write_conw(vtesc_seq::UNDER_LINE) {
        if *IS_XTERM {
            print!("{}", vtesc_seq::UNDER_LINE);
        } else {
            todo!();
        }
    }
}

pub(crate) fn cls() {
    if let Err(_) = write_conw(vtesc_seq::CLEAR_SCREEN) {
        if *IS_XTERM {
            print!("{}", vtesc_seq::CLEAR_SCREEN);
        } else {
            let stdout_handle = get_stdout_handle();
            let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::default();
            let mut scroll_rect = SMALL_RECT::default();
            let mut scroll_target = COORD::default();
            let mut fill = CHAR_INFO::default();

            unsafe {
                // Get the number of character cells in the current buffer.
                if !GetConsoleScreenBufferInfo(
                    stdout_handle,
                    &mut csbi as *mut CONSOLE_SCREEN_BUFFER_INFO,
                )
                .as_bool()
                {
                    return;
                }

                // Scroll the rectangle of the entire buffer.
                scroll_rect.Left = 0;
                scroll_rect.Top = 0;
                scroll_rect.Right = csbi.dwSize.X;
                scroll_rect.Bottom = csbi.dwSize.Y;

                // Scroll it upwards off the top of the buffer with a magnitude of the entire height.
                scroll_target.X = 0;
                scroll_target.Y = 0 - csbi.dwSize.Y;

                // Fill with empty spaces with the buffer's default text attribute.
                fill.Char.UnicodeChar = ' ' as u16;
                fill.Attributes = csbi.wAttributes;

                // Do the scroll
                ScrollConsoleScreenBufferW(
                    stdout_handle,
                    &scroll_rect,
                    null() as *const SMALL_RECT,
                    scroll_target,
                    &fill,
                );

                // Move the cursor to the top left corner too.
                csbi.dwCursorPosition.X = 0;
                csbi.dwCursorPosition.Y = 0;

                SetConsoleCursorPosition(stdout_handle, csbi.dwCursorPosition);
            }
        }
    }
}

pub(crate) fn print(r#str: &str) {
    if let Err(_) = write_conw(r#str) {
        if *IS_XTERM {
            print!("{}", r#str);
        } else {
            todo!();
        }
    }
}

pub(crate) fn println(r#str: &str) {
    print(r#str);
    print!("\n");
}
