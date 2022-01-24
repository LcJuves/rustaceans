//! https://docs.microsoft.com/zh-cn/windows/console/clearing-the-screen

#![cfg(windows)]

use core::ffi::c_void;

use std::ptr::null_mut;
use std::sync::Once;

use lazy_static::lazy_static;
use libc::wcslen;
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
// use windows::Win32::System::Console::SetConsoleScreenBufferInfoEx;
// use windows::Win32::System::Console::CONSOLE_SCREEN_BUFFER_INFOEX;

lazy_static! {
    static ref DEFAULT_WATTRIBUTES: u16 = {
        let mut buf_info = CONSOLE_SCREEN_BUFFER_INFO::default();
        let stdout_handle = &*STDOUT_HANDLE;
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

fn set_con_text_attr(wattributes: u16) {
    unsafe {
        let stdout_handle = *STDOUT_HANDLE;
        ONCE_INIT.call_once(|| {
            let _ = *DEFAULT_WATTRIBUTES;
        });
        SetConsoleTextAttribute(stdout_handle, wattributes).ok().unwrap_or(());
    }
}

fn write_conw(ansi_str: &str) -> Result<()> {
    unsafe {
        let stdout_handle = *STDOUT_HANDLE;
        ONCE_INIT.call_once(|| {
            let _ = *DEFAULT_WATTRIBUTES;
        });

        let mut mode: CONSOLE_MODE = 0;
        if !GetConsoleMode(stdout_handle, &mut mode as *mut CONSOLE_MODE).as_bool() {
            return Err(Error::from_win32());
        }

        let original_mode = mode;
        mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        if !SetConsoleMode(stdout_handle, mode).as_bool() {
            return Err(Error::from_win32());
        }

        let sequence = ansi_str.encode_utf16().collect::<Vec<u16>>();
        let mut written = 0u32;
        if !WriteConsoleW(
            stdout_handle,
            sequence.as_ptr() as *const c_void,
            wcslen(sequence.as_ptr()) as u32,
            &mut written as *mut u32,
            null_mut() as *mut c_void,
        )
        .as_bool()
        {
            // If we fail, try to restore the mode on the way out.
            SetConsoleMode(stdout_handle, original_mode);
            return Err(Error::from_win32());
        }
    }
    Ok(())
}

pub(crate) fn reset() {
    let wattributes = *DEFAULT_WATTRIBUTES;
    set_con_text_attr(wattributes);
}

pub(crate) fn set_red() {
    let wattributes = *DEFAULT_WATTRIBUTES;
    println!("wattributes >>> {:016b}", wattributes as u16);
    println!("0x4 >>> {:016b}", 0x4);
    set_con_text_attr(0x4);
}

pub(crate) fn set_green() {
    set_con_text_attr(0x2);
}

pub(crate) fn set_blue() {
    set_con_text_attr(0x1);
}

pub(crate) fn set_white() {
    set_con_text_attr(0x7);
}

// pub(crate) fn set_high_light() {
//     set_con_text_attr(FOREGROUND_INTENSITY as u16);
// }

// pub(crate) fn set_under_line() {
//     set_con_text_attr(COMMON_LVB_UNDERSCORE as u16);
// }
