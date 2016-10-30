#[macro_use]
extern crate quick_error;
extern crate regex;
extern crate serde_json;

mod context;
pub mod scaffolding;
pub mod compile;

pub use context::*;
