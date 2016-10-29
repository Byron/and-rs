#[macro_use]
extern crate quick_error;
extern crate regex;

mod context;
mod scaffolding;
pub use context::Context;
pub use scaffolding::generate_application_scaffolding;
