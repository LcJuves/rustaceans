mod cvtseq;
#[cfg(windows)]
mod nt_version_nums;
#[cfg(windows)]
mod winterm;

pub use crate::cvtseq::*;
#[cfg(windows)]
#[allow(unused_imports)]
pub(crate) use crate::{nt_version_nums::*, winterm::*};

pub fn clear_screen() {
    #[cfg(windows)]
    cls();
}
