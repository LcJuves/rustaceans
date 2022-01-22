use windows::core::Result;
use windows::Win32::Foundation::PWSTR;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, SwitchToThisWindow};

pub fn focus_window(name: &str) -> Result<()> {
    let mut lpwindowname = name.encode_utf16().collect::<Vec<u16>>();
    unsafe {
        let hwnd = FindWindowW(PWSTR::default(), PWSTR(lpwindowname.as_mut_ptr()));
        if let Err(e) = hwnd.ok() {
            if !e.code().is_ok() {
                return Err(e);
            }
        }
        SwitchToThisWindow(hwnd, true);
    }
    Ok(())
}
