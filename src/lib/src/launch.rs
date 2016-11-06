use std::path::Path;
use super::{execute_program_verbosely, BatchExecutionError, Context, find_android_executable,
            find_file_in_path, extract_tasks_for, execute_script};

pub const COMMAND_NAME: &'static str = "launch";
pub fn launch_application(at: &Path,
                          ctx: &Context,
                          emulator: &str)
                          -> Result<(), BatchExecutionError> {
    let (before, after) = extract_tasks_for(COMMAND_NAME, ctx);
    try!(execute_script(before, at));

    try!(execute_script(after, at));
    Ok(())
}
