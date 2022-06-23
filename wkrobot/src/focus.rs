#![cfg(windows)]

use windows::core::{Error, Result, PCWSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, SwitchToThisWindow};

pub fn focus_window(name: &str) -> Result<()> {
    let mut lpwindowname = name.encode_utf16().collect::<Vec<u16>>();
    unsafe {
        let hwnd = FindWindowW(PCWSTR::default(), PCWSTR(lpwindowname.as_mut_ptr()));
        if hwnd == HWND::default() {
            return Err(Error::from_win32());
        }
        SwitchToThisWindow(hwnd, true);
    }
    Ok(())
}
