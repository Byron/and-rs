#[cfg(target_os = "windows")]
pub fn path_delimiter() -> &'static str {
    ";"
}

#[cfg(not(target_os = "windows"))]
pub fn path_delimiter() -> &'static str {
    ":"
}

#[cfg(not(target_os = "windows"))]
pub fn executable_suffix() -> &'static str {
    ""
}

#[cfg(target_os = "windows")]
pub fn executable_suffix() -> &'static str {
    ".exe"
}
