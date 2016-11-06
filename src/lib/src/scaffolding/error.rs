use std::path::{Path, PathBuf};
use std::io;
use super::super::context::ContextVerificationError;

pub struct PathToWriteTo<'a>(pub &'a Path);

quick_error!{
    #[derive(Debug)]
    pub enum Error {
        Io(p: PathBuf, err: io::Error) {
            description("A file or directory could not be created")
            display("Failed to create or write '{}'", p.display())
            context(p: &'a Path, err: io::Error) -> (p.to_path_buf(), err)
            cause(err)
        }
        ExistingDirectory(dir: PathBuf) {
            description("The target directory does already exist")
            display("Cannot write new project into existing directory at '{}'", dir.display())
        }
        Write(p: PathBuf, err: io::Error) {
            description("A file or directory could not be created")
            display("Failed to create or write '{}'", p.display())
            context(p: PathToWriteTo<'a>, err: io::Error) -> (p.0.to_path_buf(), err)
            cause(err)
        }
        Context(err: ContextVerificationError) {
            description("The provided context is invalid")
            display("{}", err)
            from()
            cause(err)
        }
    }
}
