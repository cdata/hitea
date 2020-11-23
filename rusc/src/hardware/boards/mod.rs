#[cfg(feature = "hifive1_board")]
mod hifive1;

#[cfg(feature = "hifive1_board")]
pub use crate::hardware::boards::hifive1::*;
