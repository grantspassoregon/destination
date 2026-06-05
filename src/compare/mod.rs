//! The `compare` module contains data types and methods for comparing addresses.
mod compare_fire;
mod compare_wui;
mod eponym;

pub use compare_fire::*;
pub use compare_wui::{WuiMatch, WuiMatchRecord, WuiMatchRecords, WuiMatches};
pub use eponym::*;
