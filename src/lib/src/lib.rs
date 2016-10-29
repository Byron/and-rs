#[macro_use]
extern crate quick_error;
extern crate regex;
extern crate serde_json;

mod context;
mod scaffolding;
pub use context::Context;
pub use scaffolding::generate_application_scaffolding;
