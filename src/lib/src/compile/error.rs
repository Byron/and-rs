use super::super::{ExecutionError, FindError};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
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
