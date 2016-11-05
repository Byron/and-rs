use super::super::{ExecutionError, FindError};
use std::io;
use std::path::{Path, PathBuf};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ChangeCurrentWorkingDir(path: PathBuf, err: io::Error) {
            description("Failed to change current working directory")
            display("Failed to chagne current working directory to '{}'", path.display())
            context(path: &'a Path, err: io::Error) -> (path.to_path_buf(), err)
            cause(err)
        }
        Program(err: FindError) {
            description("A required executable could not be found")
            from()
            cause(err)
        }
        Execution(err: ExecutionError) {
            description("A program failed to execute")
            from()
            cause(err)
        }
    }
}
