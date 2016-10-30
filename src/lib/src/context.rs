use std::error::Error;
use std::fmt;
use regex::Regex;
use serde_json::{Error as JsonError, from_reader, to_string_pretty, Value};
use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::io::Read;

const VALID_PROJECT_NAME: &'static str = "^[0-9a-zA-Z]+$";

#[derive(PartialEq, Eq, Debug)]
pub enum ContextVerificationError {
    InvalidProjectName {
        name: String,
    },
}

#[derive(Debug)]
pub enum ContextSchemaError {
    Type {
        want: &'static str,
        field: String,
        got: Value,
    },
    MissingField {
        name: String,
    },
    Syntax(JsonError),
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

#[derive(Debug, Default, PartialEq, Eq)]
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

    pub fn deserialize(rd: &mut Read) -> Result<Context, ContextSchemaError> {
        let v: Value = try!(from_reader(rd).map_err(|err| ContextSchemaError::Syntax(err)));
        let get = |field: &str| {
            v.find(field)
                .ok_or_else(|| ContextSchemaError::MissingField { name: field.to_owned() })
                .and_then(|v| {
                    v.as_str().ok_or_else(|| {
                        ContextSchemaError::Type {
                            field: field.to_owned(),
                            want: "string",
                            got: v.to_owned(),
                        }
                    })
                })
                .map(|v| v.to_owned())
        };

        Ok(Context {
            application_name: try!(get("project")),
            package_path: try!(get("package")),
        })
    }

    pub fn serialize(&self) -> String {
        let values = [("project".to_owned(),
                       Value::String(self.application_name
                          .to_owned())),
                      ("package".to_owned(),
                       Value::String(self.package_path
                          .to_owned()))];
        let values = Value::Object(BTreeMap::from_iter(values.iter()
            .cloned()));
        to_string_pretty(&values)
            .expect("serialization to work and deal with all values we could have")
    }
}


#[cfg(test)]
mod context_serde {
    use super::Context;
    use std::io::Cursor;

    #[test]
    fn it_does_not_loose_information() {
        let ctx = Context {
            application_name: "name".to_owned(),
            package_path: "package".to_owned(),
        };

        assert_eq!(ctx,
                   Context::deserialize(&mut Cursor::new(ctx.serialize())).unwrap());
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
