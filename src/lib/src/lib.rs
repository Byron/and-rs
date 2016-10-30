#[macro_use]
extern crate quick_error;
extern crate regex;
extern crate serde_json;

mod context;
mod scaffolding;
mod compile;

pub use context::Context;
pub use scaffolding::generate_application_scaffolding;
pub use compile::compile_application;
