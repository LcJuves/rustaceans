//! https://docs.microsoft.com/zh-cn/windows/console/clearing-the-screen

#![cfg(windows)]

use core::ffi::c_void;

use std::ptr::null_mut;
use std::sync::Once;

use lazy_static::lazy_static;
use windows::core::{Error, Result};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Console::GetConsoleMode;
use windows::Win32::System::Console::GetConsoleScreenBufferInfo;
use windows::Win32::System::Console::GetStdHandle;
use windows::Win32::System::Console::SetConsoleMode;
use windows::Win32::System::Console::SetConsoleTextAttribute;
use windows::Win32::System::Console::WriteConsoleW;
use windows::Win32::System::Console::CONSOLE_MODE;
use windows::Win32::System::Console::CONSOLE_SCREEN_BUFFER_INFO;
use windows::Win32::System::Console::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
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
    if let Err(_) = write_conw("\x1b[0m") {
        let wattributes = *DEFAULT_WATTRIBUTES;
        set_con_text_attr(wattributes);
    }
}

pub(crate) fn set_red() {
    if let Err(_) = write_conw("\x1b[0;32;31m") {
        set_con_text_attr(0x4);
    }
}

pub(crate) fn set_green() {
    if let Err(_) = write_conw("\x1b[0;32;32m") {
        set_con_text_attr(0x2);
    }
}

pub(crate) fn set_blue() {
    if let Err(_) = write_conw("\x1b[0;32;34m") {
        set_con_text_attr(0x1);
    }
}

pub(crate) fn set_white() {
    if let Err(_) = write_conw("\x1b[1;37m") {
        set_con_text_attr(0x7);
    }
}

pub(crate) fn set_high_light() {
    if let Err(_) = write_conw("\x1b[1m") {
        todo!();
    }
}

pub(crate) fn set_under_line() {
    if let Err(_) = write_conw("\x1b[4m") {
        todo!();
    }
}

// pub(crate) fn clear_screen() {
//     if let Err(_) = write_conw("\x1bc") {
//         todo!();
//     }
// }

pub(crate) fn println(r#str: &str) {
    if let Err(_) = write_conw(r#str) {
        todo!();
    }
}
