mod cvtseq;
#[cfg(windows)]
mod winterm;

pub use crate::cvtseq::*;
#[cfg(windows)]
pub(crate) use crate::winterm::*;

pub fn clear_screen() {
    #[cfg(windows)]
    cls();
}
