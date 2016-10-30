use std::path::{Path, PathBuf};
use std::io;
use super::super::FindError;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(p: PathBuf, err: io::Error) {
            description("A directory could not be read")
            display("Failed to create or write '{}'", p.display())
            context(p: & 'a Path, err: io::Error) -> (p.to_path_buf(), err)
            cause(err)
        }
        Program(err: FindError) {
            from()
            cause(err)
        }
    }
}
