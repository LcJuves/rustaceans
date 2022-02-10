include!("../src/lib.rs");

#[macro_use]
mod kbd_mod;

use crate::kbd_mod::*;

use core::time::Duration;

use std::error::Error;
use std::io::{stdout, ErrorKind, Read, Write};
use std::process::{Command, Stdio};
use std::thread;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_A, VK_C, VK_E, VK_G, VK_H, VK_I,
    VK_J, VK_L, VK_N, VK_RETURN, VK_S, VK_SHIFT, VK_SPACE, VK_U, VK_V,
};

#[test]
fn test_kbd() -> Result<(), Box<dyn Error>> {
    let kbd_archive_path = _compile_kbd_exe()?;

    let mut kbdbin_child = Command::new(&kbd_archive_path).stdout(Stdio::piped()).spawn()?;

    thread::spawn(|| {
        thread::sleep(Duration::from_millis(300));
        key_press(VK_L).unwrap();
        key_press(VK_L).unwrap();
        key_press(VK_L).unwrap();
        key_press(VK_RETURN).unwrap();
    });

    let output_string = _read_child_output_string!(kbdbin_child);
    kbdbin_child.wait()?;
    assert_eq!(output_string, "lll");

    let mut kbdbin_child = Command::new(&kbd_archive_path).stdout(Stdio::piped()).spawn()?;

    thread::spawn(|| {
        thread::sleep(Duration::from_millis(500));
        key_down(VK_SHIFT).unwrap();
        key_press(VK_A).unwrap();
        key_press(VK_A).unwrap();
        key_press(VK_A).unwrap();
        key_up(VK_SHIFT).unwrap();
        key_press(VK_RETURN).unwrap();
    });

    let output_string = _read_child_output_string!(kbdbin_child);
    kbdbin_child.wait()?;
    assert_eq!(output_string, "AAA");

    let mut kbdbin_child = Command::new(&kbd_archive_path).stdout(Stdio::piped()).spawn()?;

    thread::spawn(|| {
        thread::sleep(Duration::from_millis(500));
        key_down(VK_SHIFT).unwrap();
        key_press(VK_L).unwrap();
        key_up(VK_SHIFT).unwrap();
        key_press(VK_I).unwrap();
        key_press(VK_A).unwrap();
        key_press(VK_N).unwrap();
        key_press(VK_G).unwrap();
        key_press(VK_C).unwrap();
        key_press(VK_H).unwrap();
        key_press(VK_E).unwrap();
        key_press(VK_N).unwrap();
        key_press(VK_G).unwrap();
        key_press(VK_SPACE).unwrap();
        key_down(VK_SHIFT).unwrap();
        key_press(VK_J).unwrap();
        key_up(VK_SHIFT).unwrap();
        key_press(VK_U).unwrap();
        key_press(VK_V).unwrap();
        key_press(VK_E).unwrap();
        key_press(VK_S).unwrap();
        key_press(VK_RETURN).unwrap();
    });

    let output_string = _read_child_output_string!(kbdbin_child);
    kbdbin_child.wait()?;
    assert_eq!(output_string, "Liangcheng Juves");

    let mut kbdbin_child = Command::new(&kbd_archive_path).stdout(Stdio::piped()).spawn()?;

    thread::spawn(|| {
        thread::sleep(Duration::from_millis(500));
        key_press(VK_0).unwrap();
        key_press(VK_1).unwrap();
        key_press(VK_2).unwrap();
        key_press(VK_3).unwrap();
        key_press(VK_4).unwrap();
        key_press(VK_5).unwrap();
        key_press(VK_6).unwrap();
        key_press(VK_7).unwrap();
        key_press(VK_8).unwrap();
        key_press(VK_9).unwrap();
        key_press(VK_RETURN).unwrap();
    });

    let output_string = _read_child_output_string!(kbdbin_child);
    kbdbin_child.wait()?;
    assert_eq!(output_string, "0123456789");

    std::fs::remove_file(&kbd_archive_path)?;

    Ok(())
}
