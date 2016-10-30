use regex::Regex;
use serde_json::{Error as JsonError, from_reader, to_string_pretty, Value};
use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::io::Read;

const VALID_PROJECT_NAME: &'static str = "^[0-9a-zA-Z]+$";

quick_error! {
    #[derive(PartialEq, Eq, Debug)]
    pub enum ContextVerificationError {
        InvalidProjectName {
            name: String,
        } {
            description("The project name is invalid")
            display("Project name '{}' is invalid as it does not match '{}'",
                    name, VALID_PROJECT_NAME)
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum ContextSchemaError {
        Type {
            want: &'static str,
            field: String,
            got: Value,
        } {
            description("The type of a field does not match the expected type")
            display("Could not convert value '{}' of field '{}' to type {}", got, field, want)
        }
        MissingField {
            name: String,
        } {
            description("A field was missing in the serialized context")
            display("Field '{}' was missing", name)
        }
        Deserialization(err: JsonError) {
            description("Failed to deserialize context")
            cause(err)
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
        let v: Value = try!(from_reader(rd)
            .map_err(|err| ContextSchemaError::Deserialization(err)));
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
