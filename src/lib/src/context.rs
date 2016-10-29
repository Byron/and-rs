use std::error::Error;
use std::fmt;
use regex::Regex;

const VALID_PROJECT_NAME: &'static str = "^[0-9a-zA-Z]+$";

#[derive(PartialEq, Eq, Debug)]
pub enum ContextVerificationError {
    InvalidProjectName {
        name: String,
    },
}

impl Error for ContextVerificationError {
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

#[derive(Debug, Default, Serialize)]
pub struct Context {
    pub application_name: String,
    pub package_path: String,
}

impl Context {
    pub fn verify(&self) -> Result<(), ContextVerificationError> {
        let re_valid_project_name = Regex::new(VALID_PROJECT_NAME)
            .expect("this to be a valid regex");
        if !re_valid_project_name.is_match(&self.application_name) {
            return Err(ContextVerificationError::InvalidProjectName {
                name: self.application_name.to_owned(),
            });
        }
        Ok(())
    }
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
