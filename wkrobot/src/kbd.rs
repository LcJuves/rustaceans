use core::mem::size_of;

use windows::core::{Error, Result};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VIRTUAL_KEY,
};

pub fn key_input(ki: KEYBDINPUT) -> Result<()> {
    let mut pinputs = INPUT::default();
    pinputs.r#type = INPUT_KEYBOARD;
    pinputs.Anonymous = INPUT_0 { ki: ki };
    unsafe {
        let ret = SendInput(1, &pinputs, size_of::<INPUT>() as i32);
        if ret != 1 {
            return Err(Error::from_win32());
        }
    }
    Ok(())
}

pub fn key_down(vk: VIRTUAL_KEY) -> Result<()> {
    let mut ki = KEYBDINPUT::default();
    ki.wVk = vk;

    key_input(ki)?;
    Ok(())
}

pub fn key_up(vk: VIRTUAL_KEY) -> Result<()> {
    let mut ki = KEYBDINPUT::default();
    ki.wVk = vk;
    ki.dwFlags = KEYEVENTF_KEYUP;

    key_input(ki)?;
    Ok(())
}

pub fn key_press(vk: VIRTUAL_KEY) -> Result<()> {
    key_down(vk)?;
    key_up(vk)?;
    Ok(())
}
