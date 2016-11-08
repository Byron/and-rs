use std::path::{Path, PathBuf};
use std::io;
use super::Context;
use super::process::{FindError, ExecutionError, execute_shell_script_verbosely};

pub fn android_platform_jar_path(android_home_dir: &Path, ctx: &Context) -> String {
    format!("{}/platforms/{}/android.jar",
            android_home_dir.display(),
            ctx.target)
}

pub fn extract_tasks_for<'a>(command: &'static str,
                             ctx: &'a Context)
                             -> (Option<&'a String>, Option<&'a String>) {
    ctx.tasks
        .get(command)
        .map(|t| (t.before.as_ref(), t.after.as_ref()))
        .unwrap_or((None, None))
}


pub fn execute_script(script: Option<&String>, at: &Path) -> Result<(), BatchExecutionError> {
    if let Some(script) = script {
        try!(execute_shell_script_verbosely(at, script));
    };
    Ok(())
}



quick_error! {
    #[derive(Debug)]
    pub enum BatchExecutionError {
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
