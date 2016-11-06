use std::path::{PathBuf, Path};
use super::{ChangeCWD, find_file_in_path, find_android_executable, execute_program_verbosely,
            Context, path_delimiter, BatchExecutionError, android_platform_jar_path,
            execute_program_verbosely_with_task};
use glob::glob;
use quick_error::ResultExt;

pub const COMMAND_NAME: &'static str = "compile";

pub fn compile_application(at: &Path, ctx: &Context) -> Result<(), BatchExecutionError> {
    let (aapt_path, android_home_dir) = try!(find_android_executable("aapt"));
    let javac_path = try!(find_file_in_path("javac"));
    let android_jar_path = android_platform_jar_path(&android_home_dir, ctx);
    try!(execute_program_verbosely_with_task(ctx.tasks.get(COMMAND_NAME),
                                             at,
                                             &aapt_path,
                                             &["package",
                                               "-vfm",
                                               "-S",
                                               "res",
                                               "-J",
                                               "src",
                                               "-M",
                                               "AndroidManifest.xml",
                                               "-I",
                                               &android_jar_path]));

    let classpath = format!("{}{}obj", android_jar_path, path_delimiter());
    let source_files: Vec<_> = {
        let _in_project_dir = try!(ChangeCWD::into(at).context(at));
        glob("src/**/*.java")
            .expect("valid glob")
            .filter_map(Result::ok)
            .collect()
    };

    const LANGUAGE_LEVEL: &'static str = "1.7";
    let mut args = vec!["-verbose",
                        "-source",
                        LANGUAGE_LEVEL,
                        "-target",
                        LANGUAGE_LEVEL,
                        "-d",
                        "obj",
                        "-classpath",
                        &classpath,
                        "-sourcepath",
                        "src"];
    for valid_java_path in source_files.iter().map(PathBuf::as_path).filter_map(Path::to_str) {
        args.push(valid_java_path);
    }
    let args = args;
    try!(execute_program_verbosely(at, &javac_path, &args));
    Ok(())
}
