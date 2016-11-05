use std::path::Path;
use super::{BatchExecutionError, Context, find_android_executable, find_file_in_path};

pub fn package_application(at: &Path, ctx: &Context) -> Result<(), BatchExecutionError> {
    let (dx_path, _) = try!(find_android_executable("dx"));
    let (aapt_path, _) = try!(find_android_executable("aapt"));
    let (zipalign_path, _) = try!(find_android_executable("zipalign"));
    let jarsigner_path = try!(find_file_in_path("jarsigner"));

    Ok(())
}
