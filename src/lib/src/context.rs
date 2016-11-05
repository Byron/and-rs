use regex::Regex;
use std::io::{self, Read};
use rustc_serialize::json::{decode, as_pretty_json, DecoderError};
use std::collections::HashMap;

const VALID_PROJECT_NAME: &'static str = "^[0-9a-zA-Z]+$";
const VALID_TARGET_NAME: &'static str = "^[0-9a-zA-Z_-]+$";

quick_error! {
    #[derive(PartialEq, Eq, Debug)]
    pub enum ContextVerificationError {
        InvalidTargetName (name: String) {
            description("The target name is invalid")
            display("Target name '{}' is invalid as it does not match '{}'",
                    name, VALID_TARGET_NAME)
        }
        InvalidProjectName (name: String) {
            description("The project name is invalid")
            display("Project name '{}' is invalid as it does not match '{}'",
                    name, VALID_PROJECT_NAME)
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum ContextDeserializationError {
        Io (err: io::Error) {
            description("Failed to read from stream")
            from()
            cause(err)
        }
        Deserialization(err: DecoderError) {
            description("Failed to deserialize context")
            from()
            cause(err)
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct Task {
    pub before: Option<String>,
    pub after: Option<String>
}

#[derive(Debug, Default, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct Context {
    pub project: String,
    pub package: String,
    pub target: String,
    pub tasks: HashMap<String, Task>
}

impl Context {
    pub fn verify(&self) -> Result<(), ContextVerificationError> {
        let re_valid_target_name = Regex::new(VALID_TARGET_NAME).expect("this to be a valid regex");
        let re_valid_project_name = Regex::new(VALID_PROJECT_NAME)
            .expect("this to be a valid regex");
        if !re_valid_project_name.is_match(&self.project) {
            return Err(ContextVerificationError::InvalidProjectName(self.project
                .to_owned()));
        }
        if !re_valid_target_name.is_match(&self.target) {
            return Err(ContextVerificationError::InvalidTargetName(self.target.to_owned()));
        }
        Ok(())
    }

    pub fn deserialize(rd: &mut Read) -> Result<Context, ContextDeserializationError> {
        let mut buf = String::with_capacity(128);
        try!(rd.read_to_string(&mut buf));
        Ok(try!(decode(&buf)))
    }

    pub fn serialize(&self) -> String {
        format!("{}", as_pretty_json(self))
    }
}
