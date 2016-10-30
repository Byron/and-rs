use std::path::Path;
use super::super::{find_android_executable, Context};
use super::Error;
use std::process::{ExitStatus, Command};

pub fn compile_application(at: &Path, ctx: &Context) -> Result<(), Error> {
    let aapt_path = try!(find_android_executable("aapt"));
    let status: ExitStatus =
        try!(Command::new(&aapt_path).args(&["--foo"]).status().map_err(|err| {
            Error::Spawn {
                path: aapt_path,
                err: err,
            }
        }));
    Ok(())
}
