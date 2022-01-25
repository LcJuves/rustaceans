mod winterm;

pub(crate) use crate::winterm::*;

pub fn clear_screen() {
    #[cfg(windows)]
    {
        use crate::winterm::get_stdout_handle;

        use std::ptr::null;

        use windows::Win32::System::Console::GetConsoleScreenBufferInfo;
        use windows::Win32::System::Console::ScrollConsoleScreenBufferW;
        use windows::Win32::System::Console::SetConsoleCursorPosition;
        use windows::Win32::System::Console::CHAR_INFO;
        use windows::Win32::System::Console::CONSOLE_SCREEN_BUFFER_INFO;
        use windows::Win32::System::Console::COORD;
        use windows::Win32::System::Console::SMALL_RECT;

        let stdout_handle = get_stdout_handle();
        let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = CONSOLE_SCREEN_BUFFER_INFO::default();
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
