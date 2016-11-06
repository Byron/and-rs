use std::path::{PathBuf, Path};
use super::{execute_program_verbosely, BatchExecutionError, Context, find_android_executable,
            find_file_in_path, android_platform_jar_path, get_env_as_path, FindError,
            execute_program_verbosely_with_task};

pub const COMMAND_NAME: &'static str = "package";

fn fetch_or_create_android_keystore() -> Result<PathBuf, FindError> {
    const ANDROID_KEYSTORE_NAME: &'static str = "debug.keystore";
    let home = try!(get_env_as_path("HOME"));
    let dir = home.join(".android");
    let keystore = dir.join(ANDROID_KEYSTORE_NAME);
    if keystore.is_file() {
        Ok(keystore)
    } else {
        let not_found = || {
            FindError::NotFound {
                name: ANDROID_KEYSTORE_NAME.to_owned(),
                dir: dir.to_owned(),
            }
        };

        find_file_in_path("keytool")
            .and_then(|keytool_path| {
                execute_program_verbosely(Path::new("."),
                                          &keytool_path,
                                          &["-genkey",
                                            "-v",
                                            "-keystore",
                                            &keystore.to_string_lossy(),
                                            "-storepass",
                                            "android",
                                            "-keypass",
                                            "android",
                                            "-alias",
                                            "androiddebugkey",
                                            "-dname",
                                            "CN=Android Debug,O=Android,C=US"])
                    .map(|_| keystore)
                    .map_err(|_| not_found())
            })
            .map_err(|_| not_found())
    }
}

pub fn package_application(at: &Path, ctx: &Context) -> Result<(), BatchExecutionError> {
    let (dx_path, android_home_dir) = try!(find_android_executable("dx"));
    let (aapt_path, _) = try!(find_android_executable("aapt"));
    let (zipalign_path, _) = try!(find_android_executable("zipalign"));
    let jarsigner_path = try!(find_file_in_path("jarsigner"));
    let debug_keystore_path = try!(fetch_or_create_android_keystore());

    try!(execute_program_verbosely(at,
                                   &dx_path,
                                   &["--dex",
                                     "--verbose",
                                     "--output",
                                     "bin/classes.dex",
                                     "obj",
                                     "lib"]));

    let android_jar_path = android_platform_jar_path(&android_home_dir, ctx);
    let unsigned_apk_path = format!("bin/{}.unsigned.apk", ctx.project);
    try!(execute_program_verbosely(at,
                                   &aapt_path,
                                   &["package",
                                     "-vf",
                                     "-M",
                                     "AndroidManifest.xml",
                                     "-S",
                                     "res",
                                     "-I",
                                     &android_jar_path,
                                     "-F",
                                     &unsigned_apk_path,
                                     "bin"]));

    let signed_apk_path = format!("bin/{}.signed.apk", ctx.project);
    try!(execute_program_verbosely(at,
                                   &jarsigner_path,
                                   &["-verbose",
                                     "-storepass",
                                     "android",
                                     "-keypass",
                                     "android",
                                     "-keystore",
                                     &debug_keystore_path.to_string_lossy(),
                                     "-signedjar",
                                     &signed_apk_path,
                                     &unsigned_apk_path,
                                     "androiddebugkey"]));

    try!(execute_program_verbosely_with_task(ctx.tasks.get("package"),
                                             at,
                                             &zipalign_path,
                                             &["-v",
                                               "-f",
                                               "4",
                                               &signed_apk_path,
                                               &format!("bin/{}.apk", ctx.project)]));

    Ok(())
}
