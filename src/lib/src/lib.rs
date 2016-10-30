#[macro_use]
extern crate quick_error;
extern crate regex;
extern crate serde_json;
extern crate walkdir;

mod context;
mod process;
pub mod scaffolding;
pub mod compile;

pub use context::*;
pub use process::*;
