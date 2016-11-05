use std::path::Path;
use super::super::{find_android_executable, execute_program_verbosely, Context};
use super::Error;

pub fn compile_application(at: &Path, ctx: &Context) -> Result<(), Error> {
    let aapt_path = try!(find_android_executable("aapt"));
    try!(execute_program_verbosely(at,
                                   &aapt_path,
                                   &["-vfm",
                                     "-S",
                                     "res",
                                     "-J",
                                     "src",
                                     "-M",
                                     "AndroidManifest.xml",
                                     "-I"]));
    Ok(())
}
