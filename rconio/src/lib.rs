mod winterm;

pub(crate) use crate::winterm::*;

pub fn clear_screen() {
    #[cfg(windows)]
    winterm::cls();
}
