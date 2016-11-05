use std::path::{Path, PathBuf};
use walkdir::{Error as IterationError, WalkDir};
use std::env;
use std::io::{self, Write};
use std::ffi::OsStr;
use std::process::{ExitStatus, Command};
use super::{executable_suffix, path_delimiter};


quick_error! {
    #[derive(Debug)]
    pub enum FindError {
        Variable {
         name: &'static str,
         err: env::VarError
        } {
            description("Environment variable missing or invalid")
            display("The {} environment variable is not valid", name)
            cause(err)
        }
        Iteration{ dir: PathBuf, err: IterationError } {
            description("Failed to traverse directory")
            display("Directory '{}' could not be traversed", dir.display())
            cause(err)
        }
        NotFound{dir: PathBuf, name: String} {
            description("executable not found")
            display("An executable named '{}' could not be found under '{}'", name, dir.display())
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum ExecutionError {
        Spawn{ path: PathBuf, err: io::Error } {
            description("A program could not be spawned")
            display("Failed to start '{}'", path.display())
            cause(err)
        }
        Exit{executable: PathBuf, args: Vec<String>, status: ExitStatus} {
            description("Program exited with non-zero code.")
            display("Program invocation `{} {}` failed with exit code {}",
                        executable.display(),
                        &args.join(" "),
                        status.code().expect("exit code when program is done"))
        }
    }
}

pub fn find_in_path(name: &str) -> Result<PathBuf, FindError> {
    get_env_as_path("PATH").and_then(|path| {
        path.to_string_lossy()
            .split(path_delimiter())
            .map(Path::new)
            .map(|subpath| find_executable(subpath, name))
            .filter_map(Result::ok)
            .next()
            .ok_or_else(|| {
                FindError::NotFound {
                    name: name.to_owned(),
                    dir: path,
                }
            })
    })
}

pub fn find_executable(root: &Path, name: &str) -> Result<PathBuf, FindError> {
    let name = {
        let mut n = name.to_owned();
        n.push_str(executable_suffix());
        n
    };
    for entry in WalkDir::new(root) {
        match entry {
            Ok(entry) => {
                match entry.path()
                    .file_name()
                    .map(OsStr::to_str)
                    .expect("conversion to OsStr to work") {
                    Some(file_name) if file_name == name => return Ok(entry.path().to_owned()),
                    Some(_) | None => continue,
                }
            }
            Err(err) => {
                return Err(FindError::Iteration {
                    dir: err.path().unwrap_or(root).to_owned(),
                    err: err,
                })
            }
        }
    }
    Err(FindError::NotFound {
        name: name.to_owned(),
        dir: root.to_owned(),
    })
}

pub fn get_env_as_path(name: &'static str) -> Result<PathBuf, FindError> {
    env::var(name)
        .map_err(|err| {
            FindError::Variable {
                name: name,
                err: err,
            }
        })
        .map(PathBuf::from)
}

pub fn find_android_executable(name: &str) -> Result<(PathBuf, PathBuf), FindError> {
    get_env_as_path("ANDROID_HOME")
        .and_then(|root| find_executable(&root.join("build-tools"), name).map(|exe| (exe, root)))
}

pub fn execute_program_verbosely(at_dir: &Path,
                                 executable: &Path,
                                 args: &[&str])
                                 -> Result<(), ExecutionError> {
    write!(io::stderr(),
           "{} >>> {} {}\n",
           at_dir.display(),
           executable.display(),
           args.join(" "))
        .ok();
    let status: ExitStatus = try!(Command::new(executable)
        .current_dir(at_dir)
        .args(args)
        .status()
        .map_err(|err| {
            ExecutionError::Spawn {
                path: executable.to_owned(),
                err: err,
            }
        }));

    if status.success() {
        Ok(())
    } else {
        Err(ExecutionError::Exit {
            executable: executable.to_owned(),
            args: args.iter().cloned().map(String::from).collect(),
            status: status,
        })
    }
}

pub struct ChangeCWD {
    previous_cwd: PathBuf,
}

impl ChangeCWD {
    pub fn into(dir: &Path) -> Result<ChangeCWD, io::Error> {
        let res = ChangeCWD { previous_cwd: try!(env::current_dir()) };
        try!(env::set_current_dir(dir));
        Ok(res)
    }
}

impl Drop for ChangeCWD {
    fn drop(&mut self) {
        env::set_current_dir(&self.previous_cwd).ok();
    }
}
