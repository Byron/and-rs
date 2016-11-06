use std::path::Path;
use super::{execute_program_verbosely, BatchExecutionError, Context, find_android_executable,
            extract_tasks_for, execute_script};

pub const COMMAND_NAME: &'static str = "launch";
pub fn launch_application(at: &Path, ctx: &Context) -> Result<(), BatchExecutionError> {
    let (before, after) = extract_tasks_for(COMMAND_NAME, ctx);
    let (adb_path, _) = try!(find_android_executable("adb"));
    try!(execute_script(before, at));
    try!(execute_program_verbosely(at,
                                   &adb_path,
                                   &["-e", "install", &format!("bin/{}.apk", ctx.project)]));
    try!(execute_script(after, at));
    Ok(())
}
