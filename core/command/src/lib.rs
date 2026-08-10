#![forbid(unsafe_code)]
extern crate alloc;
extern crate core;
// ########################################

pub mod arguments;
pub mod commands;
#[cfg(feature = "std")]
pub mod executors;

pub use arguments::delimited_string::{DelimitedString, Delimiter};
