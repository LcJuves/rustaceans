mod vtesc_seq;
mod winterm;

pub use crate::vtesc_seq::*;
pub(crate) use crate::winterm::*;

pub fn clear_screen() {
    #[cfg(windows)]
    winterm::cls();
}
