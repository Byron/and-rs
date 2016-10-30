use std::path::Path;
use super::super::{find_android_executable, Context};
use super::Error;

pub fn compile_application(at: &Path, ctx: &Context) -> Result<(), Error> {
    let aapt_path = try!(find_android_executable("aapt"));
    Ok(())
}
