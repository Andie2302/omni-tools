#![no_std]
#![forbid(unsafe_code)]
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod argument;
pub mod argument_builder;
pub mod arguments;
pub mod command;
pub mod delimited_string;

pub use argument::Argument;
pub use argument_builder::ArgumentBuilder;
pub use arguments::Arguments;
pub use command::Command;
pub use delimited_string::{DelimitedString, Delimiter};