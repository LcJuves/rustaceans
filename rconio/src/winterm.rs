//! https://docs.microsoft.com/zh-cn/windows/console/clearing-the-screen
//! http://www.cplusplus.com/articles/2ywTURfi/

#![cfg(windows)]

use crate::cvtseq;

use core::ffi::c_void;
use core::mem::size_of;
use core::slice;

use std::ptr::{null, null_mut};
use std::sync::Once;

use lazy_static::lazy_static;
use windows::core::{Error, Result};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::Storage::FileSystem::FileNameInfo;
use windows::Win32::Storage::FileSystem::GetFileInformationByHandleEx;
use windows::Win32::Storage::FileSystem::FILE_NAME_INFO;
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
use windows::Win32::System::Console::FOREGROUND_INTENSITY;
use windows::Win32::System::Console::SMALL_RECT;
use windows::Win32::System::Console::STD_OUTPUT_HANDLE;

lazy_static! {
    static ref DEFAULT_WATTRIBUTES: u16 = get_curr_wattributes();
    static ref IS_MINTTY: bool = check_is_mintty();
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

fn get_curr_wattributes() -> u16 {
    let mut buf_info = CONSOLE_SCREEN_BUFFER_INFO::default();
    let stdout_handle = *STDOUT_HANDLE;
    unsafe {
        GetConsoleScreenBufferInfo(stdout_handle, &mut buf_info as *mut CONSOLE_SCREEN_BUFFER_INFO)
            .ok()
            .unwrap_or(());
    }
    buf_info.wAttributes
}

// https://github.com/softprops/atty/blob/master/src/lib.rs
fn check_is_mintty() -> bool {
    let stdout_handle = *STDOUT_HANDLE;
    let size = size_of::<FILE_NAME_INFO>();
    type WCHAR = u16;
    let mut name_info_bytes = vec![0u8; size + (MAX_PATH as usize) * size_of::<WCHAR>()];
    let res = unsafe {
        GetFileInformationByHandleEx(
            stdout_handle,
            FileNameInfo,
            name_info_bytes.as_mut_ptr() as *mut c_void,
            name_info_bytes.len() as u32,
        )
    };

    if !res.as_bool() {
        return false;
    }
    let name_info: &FILE_NAME_INFO =
        unsafe { &*(name_info_bytes.as_ptr() as *const FILE_NAME_INFO) };
    let s = unsafe {
        slice::from_raw_parts(name_info.FileName.as_ptr(), name_info.FileNameLength as usize / 2)
    };
    let name = String::from_utf16_lossy(s);
    // This checks whether 'pty' exists in the file name, which indicates that
    // a pseudo-terminal is attached. To mitigate against false positives
    // (e.g., an actual file name that contains 'pty'), we also require that
    // either the strings 'msys-' or 'cygwin-' are in the file name as well.)
    let is_msys = name.contains("msys-") || name.contains("cygwin-");
    let is_pty = name.contains("-pty");
    is_msys && is_pty
}

fn set_con_text_attr(wattributes: u16) {
    unsafe {
        let stdout_handle = get_stdout_handle();
        // FIXME
        let default_wattributes = *DEFAULT_WATTRIBUTES;
        let default_wattr_bg_color = wattr_bg_color(default_wattributes);
        let default_wattr_fg_color = wattr_fg_color(default_wattributes);
        // dbg!(default_wattr_bg_color);
        // dbg!(default_wattr_fg_color);

        let wattr_bg_color = wattr_bg_color(wattributes);
        let wattr_fg_color = wattr_fg_color(wattributes);
        // dbg!(wattr_bg_color);
        // dbg!(wattr_fg_color);

        let mut _wattributes: u16 = wattributes;
        if wattr_bg_color == 0 {
            _wattributes = default_wattr_bg_color << 4 | _wattributes;
        }

        if wattr_fg_color == 0 {
            _wattributes = default_wattr_fg_color | _wattributes;
        }
        // END FIXME

        SetConsoleTextAttribute(stdout_handle, _wattributes).ok().unwrap_or(());
    }
}

#[inline]
fn wattr_fg_color(wattributes: u16) -> u16 {
    wattributes % 16
}

#[inline]
fn wattr_bg_color(wattributes: u16) -> u16 {
    (wattributes / 16) % 16
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
    if *IS_MINTTY {
        print!("{}", cvtseq::CSI_NONE);
    } else {
        let wattributes = *DEFAULT_WATTRIBUTES;
        set_con_text_attr(wattributes);
    }
}

pub(crate) fn set_red() {
    if *IS_MINTTY {
        print!("{}", cvtseq::CSI_RED);
    } else {
        if let Err(_) = write_conw(cvtseq::CSI_RED) {
            set_con_text_attr(0x04 /* RED */);
        }
    }
}

pub(crate) fn set_green() {
    if *IS_MINTTY {
        print!("{}", cvtseq::CSI_GREEN);
    } else {
        if let Err(_) = write_conw(cvtseq::CSI_GREEN) {
            set_con_text_attr(0x2 /* GREEN */);
        }
    }
}

pub(crate) fn set_blue() {
    if *IS_MINTTY {
        print!("{}", cvtseq::CSI_BLUE);
    } else {
        if let Err(_) = write_conw(cvtseq::CSI_BLUE) {
            set_con_text_attr(0x1 /* BLUE */);
        }
    }
}

pub(crate) fn set_white() {
    if *IS_MINTTY {
        print!("{}", cvtseq::CSI_WHITE);
    } else {
        if let Err(_) = write_conw(cvtseq::CSI_WHITE) {
            set_con_text_attr(0x7 /* WHITE */);
        }
    }
}

pub(crate) fn set_high_light() {
    if *IS_MINTTY {
        print!("{}", cvtseq::CSI_HIGH_LIGHT);
    } else {
        if let Err(_) = write_conw(cvtseq::CSI_HIGH_LIGHT) {
            let mut wattributes = get_curr_wattributes();
            let wattr_bg_color = wattr_bg_color(wattributes);
            let wattr_fg_color = wattr_fg_color(wattributes);
            wattributes = wattr_bg_color << 4 | (FOREGROUND_INTENSITY as u16 | wattr_fg_color);
            set_con_text_attr(wattributes);
        }
    }
}

pub(crate) fn set_under_line() {
    if *IS_MINTTY {
        print!("{}", cvtseq::CSI_UNDER_LINE);
    } else {
        if let Err(_) = write_conw(cvtseq::CSI_UNDER_LINE) {
            todo!();
        }
    }
}

pub(crate) fn cls() {
    if *IS_MINTTY {
        print!("{}", cvtseq::ESC_CLEAR_SCREEN);
    } else {
        if let Err(_) = write_conw(cvtseq::ESC_CLEAR_SCREEN) {
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
    if *IS_MINTTY {
        print!("{}", r#str);
    } else {
        if let Err(_) = write_conw(r#str) {
            todo!();
        }
    }
}

pub(crate) fn println(r#str: &str) {
    print(r#str);
    print!("\n");
}
