#[macro_use]
extern crate quick_error;

use std::error::Error as ErrorTrait;
use std::path::{Path, PathBuf};
use std::io;

quick_error!{
    #[derive(Debug)]
    pub enum Error {
        Io(p: PathBuf, err: io::Error) {
            description("A file or directory could not be created")
            display("Failed to create or write '{}'", p.display())
            context(p: &'a Path, err: io::Error) -> (p.to_path_buf(), err)
            cause(err)
        }
        Other(p: PathBuf, err: Box<ErrorTrait>) {
            description("Any other error that we don't necessarily know")
            display("An error occurred: {}", err)
            cause(&**err)
        }
    }
}

pub struct Context {
    pub application_name: String,
    pub package_path: String
}

pub fn generate_application_scaffolding(ctx: Context) -> Result<(), Error> {
    Ok(())
}
