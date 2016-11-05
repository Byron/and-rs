use std::path::{PathBuf, Path};
use super::{execute_program_verbosely, BatchExecutionError, Context, find_android_executable,
            find_file_in_path, android_platform_jar_path, get_env_as_path, FindError};

fn verified_android_keystore_path() -> Result<PathBuf, FindError> {
    get_env_as_path("HOME").and_then(|home| {
        const ANDROID_KEYSTORE_NAME: &'static str = "debug.keystore";
        let dir = home.join(".android", );
        let keystore = dir.join(ANDROID_KEYSTORE_NAME);
        if keystore.is_file() {
            Ok(keystore)
        } else {
            Err(FindError::NotFound {
                dir: dir,
                name: ANDROID_KEYSTORE_NAME.to_owned()
            })
        }
    })
}

pub fn package_application(at: &Path, ctx: &Context) -> Result<(), BatchExecutionError> {
    let (dx_path, android_home_dir) = try!(find_android_executable("dx"));
    let (aapt_path, _) = try!(find_android_executable("aapt"));
    let (zipalign_path, _) = try!(find_android_executable("zipalign"));
    let jarsigner_path = try!(find_file_in_path("jarsigner"));
    let debug_keystore_path = try!(verified_android_keystore_path());

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
    try!(execute_program_verbosely(at, &aapt_path, &[
        "package",
        "-vf",
        "-M", "AndroidManifest.xml",
        "-S", "res",
        "-I", &android_jar_path,
        "-F", &unsigned_apk_path,
        "bin"
    ]));

    let signed_apk_path = format!("bin/{}.signed.apk", ctx.project);
    try!(execute_program_verbosely(at, &jarsigner_path, &[
        "-verbose",
        "-storepass", "android",
        "-keypass", "android",
        "-keystore", &debug_keystore_path.to_string_lossy(),
        "-signedjar", &signed_apk_path,
        &unsigned_apk_path,
        "androiddebugkey"
    ]));

    try!(execute_program_verbosely(at, &zipalign_path, &[
        "-v",
        "-f", "4",
        &signed_apk_path,
        &format!("bin/{}.apk", ctx.project)
    ]));

    Ok(())
}
