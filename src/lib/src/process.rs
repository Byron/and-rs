use std::path::{Path, PathBuf};
use walkdir::{Error as IterationError, WalkDir};
use std::env;
use std::ffi::OsStr;


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

pub fn find_executable(root: &Path, name: &str) -> Result<PathBuf, FindError> {
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

pub fn find_android_executable(name: &str) -> Result<PathBuf, FindError> {
    const ANDROID_HOME: &'static str = "ANDROID_HOME";
    env::var(ANDROID_HOME)
        .map_err(|err| {
            FindError::Variable {
                name: ANDROID_HOME,
                err: err,
            }
        })
        .map(PathBuf::from)
        .and_then(|root| find_executable(&root, name))
}
