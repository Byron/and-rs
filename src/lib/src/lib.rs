#[macro_use]
extern crate quick_error;
extern crate regex;
extern crate walkdir;
extern crate rustc_serialize;
extern crate glob;

mod context;
mod process;
mod os;

pub mod scaffolding;
pub mod compile;

pub use os::*;
pub use context::*;
pub use process::*;
