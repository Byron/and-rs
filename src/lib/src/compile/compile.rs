use std::path::{PathBuf, Path};
use super::super::{ChangeCWD, find_file_in_path, find_android_executable,
                   execute_program_verbosely, Context};
use super::Error;
use super::super::path_delimiter;
use glob::glob;
use quick_error::ResultExt;

pub fn compile_application(at: &Path, ctx: &Context) -> Result<(), Error> {
    let (aapt_path, android_home_dir) = try!(find_android_executable("aapt"));
    let javac_path = try!(find_file_in_path("javac"));
    let android_jar_path = format!("{}/platforms/{}/android.jar",
                                   android_home_dir.display(),
                                   ctx.target);
    try!(execute_program_verbosely(at,
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
    let mut args = vec!["-verbose", "-d", "obj", "-classpath", &classpath, "-sourcepath", "src"];
    for valid_java_path in source_files.iter().map(PathBuf::as_path).filter_map(Path::to_str) {
        args.push(valid_java_path);
    }
    let args = args;
    try!(execute_program_verbosely(at, &javac_path, &args));
    Ok(())
}
