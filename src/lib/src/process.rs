use std::path::PathBuf;

quick_error! {
    #[derive(PartialEq, Eq, Debug)]
    pub enum FindError {
        AndroidHomeUnset {
            description("The ANDROID_HOME environment variable is not set")
        }
    }
}

pub fn find_android_executable(name: &str) -> Result<PathBuf, FindError> {
    Ok(PathBuf::new())
}
