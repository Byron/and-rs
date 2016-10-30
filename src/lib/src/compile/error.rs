use std::path::{Path, PathBuf};
use std::io;
use super::super::FindError;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Spawn{ path: PathBuf, err: io::Error } {
            description("A program could not be spawned")
            display("Failed to start '{}'", path.display())
            cause(err)
        }
        Program(err: FindError) {
            from()
            cause(err)
        }
    }
}
