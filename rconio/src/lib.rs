mod cvtseq;
mod winterm;

pub use crate::cvtseq::*;
pub(crate) use crate::winterm::*;

pub fn clear_screen() {
    #[cfg(windows)]
    cls();
}
