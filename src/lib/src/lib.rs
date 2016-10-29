#[macro_use]
extern crate quick_error;
extern crate regex;

use quick_error::ResultExt;
use regex::{Captures, Regex};
use std::error::Error as ErrorTrait;
use std::path::{Path, PathBuf};
use std::fmt;
use std::io::{self, Write};
use std::fs::{File, create_dir_all};

struct PathToWriteTo<'a>(&'a Path);

const VALID_PROJECT_NAME: &'static str = "^[0-9a-zA-Z]+$";
const SUBTITUTION_KEY: &'static str = r#"\$\{(\w+)\}"#;

quick_error!{
    #[derive(Debug)]
    pub enum Error {
        Io(p: PathBuf, err: io::Error) {
            description("A file or directory could not be created")
            display("Failed to create or write '{}'", p.display())
            context(p: &'a Path, err: io::Error) -> (p.to_path_buf(), err)
            cause(err)
        }
        Write(p: PathBuf, err: io::Error) {
            description("A file or directory could not be created")
            display("Failed to create or write '{}'", p.display())
            context(p: PathToWriteTo<'a>, err: io::Error) -> (p.0.to_path_buf(), err)
            cause(err)
        }
        Context(err: ContextVerificationError) {
            description("The provided context is invalid")
            display("{}", err)
            from()
            cause(err)
        }
        Other(p: PathBuf, err: Box<ErrorTrait>) {
            description("Any other error that we don't necessarily know")
            display("An error occurred: {}", err)
            cause(&**err)
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ContextVerificationError {
    InvalidProjectName {
        name: String,
    },
}

impl ErrorTrait for ContextVerificationError {
    fn description(&self) -> &str {
        "The context contained invalid values"
    }
}

impl fmt::Display for ContextVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ContextVerificationError::InvalidProjectName { ref name } => {
                write!(f,
                       "Project name '{}' is invalid as it does not match '{}'",
                       name,
                       VALID_PROJECT_NAME)
            }
        }
    }
}

pub struct Context {
    pub application_name: String,
    pub package_path: String,
}

impl Context {
    pub fn verify(&self) -> Result<(), ContextVerificationError> {
        let re_valid_project_name = Regex::new(VALID_PROJECT_NAME).expect("this to be a valid regex");
        if !re_valid_project_name.is_match(&self.application_name) {
            return Err(ContextVerificationError::InvalidProjectName {
                name: self.application_name.to_owned(),
            });
        }
        Ok(())
    }
}

fn dotted_package_name_to_package_path(name: &str) -> String {
    Path::new("src").join(name.replace(".", "/")).to_string_lossy().into_owned()
}

fn strip_heredoc(mut here: &str) -> &str {
    here = &here[here.find('\n').expect("LF and first line") + 1..];
    &here[..here.rfind('\n').expect("LF and last line")]
}

fn manifest_content(ctx: &Context) -> String {
    let content = strip_heredoc(include_str!("./assets/manifest.cr"));
    let re: Regex = Regex::new(SUBTITUTION_KEY).expect("valid regex literal");
    re.replace_all(content, |c: &Captures| {
        match c.at(1).expect("single capture") {
            "package" => ctx.package_path.to_owned(),
            "project" => ctx.application_name.to_owned(),
            x => panic!("handle unknown variable: {}", x)
        }
    })
}

fn write_utf8_file(contents: &str, path: &Path) -> Result<(), Error> {
    let mut f: File = try!(File::create(path).context(path));
    try!(f.write(contents.as_bytes()).context(PathToWriteTo(path)));
    Ok(())
}

pub fn generate_application_scaffolding(ctx: &Context) -> Result<(), Error> {
    try!(ctx.verify());
    let app_path = |path: &str| Path::new(&ctx.application_name).join(path);
    let package_dir = app_path(&dotted_package_name_to_package_path(&ctx.package_path));
    try!(create_dir_all(&package_dir).context(package_dir.as_path()));
    try!(write_utf8_file(&manifest_content(ctx), &app_path("AndroidManifest.xml")));
    Ok(())
}

#[cfg(test)]
mod context_verification_project_name {
    use super::{ContextVerificationError, Context};

    fn project_ctx(name: &str) -> Context {
        Context {
            application_name: name.to_owned(),
            package_path: "package".to_owned(),
        }
    }

    #[test]
    fn it_likes_latin_characters() {
        let name = "5HelloWorld123";
        assert_eq!(project_ctx(name).verify(), Ok(()));
    }

    #[test]
    fn it_rejects_non_latin_literals() {
        let name = "$1hi!";
        assert_eq!(project_ctx(name).verify(),
                   Err(ContextVerificationError::InvalidProjectName { name: name.to_owned() }));
    }

    #[test]
    fn it_rejects_dashes() {
        let name = "Hello-World";
        assert_eq!(project_ctx(name).verify(),
                   Err(ContextVerificationError::InvalidProjectName { name: name.to_owned() }));
    }
}

#[test]
fn test_dotted_package_name_to_package_path() {
    assert_eq!(dotted_package_name_to_package_path("hello.wonderful.world"),
               "src/hello/wonderful/world");
}
