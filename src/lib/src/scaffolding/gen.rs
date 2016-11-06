use quick_error::ResultExt;
use regex::{Captures, Regex};
use std::path::Path;
use std::io::Write;
use std::fs::{File, create_dir_all};

use super::super::Context;
use super::error::{PathToWriteTo, Error};

const SUBTITUTION_KEY: &'static str = r#"\$\{(\w+)\}"#;

fn dotted_package_name_to_package_path(name: &str) -> String {
    Path::new("src").join(name.replace(".", "/")).to_string_lossy().into_owned()
}

fn strip_heredoc(mut here: &str) -> &str {
    here = &here[here.find('\n').expect("LF and first line") + 1..];
    &here[..here.rfind('\n').expect("LF and last line")]
}

fn substitute_context(content: &str, ctx: &Context) -> String {
    let re: Regex = Regex::new(SUBTITUTION_KEY).expect("valid regex literal");
    re.replace_all(content, |c: &Captures| {
        match c.at(1).expect("single capture") {
            "package" => ctx.package.to_owned(),
            "project" => ctx.project.to_owned(),
            x => panic!("handle unknown variable: {}", x),
        }
    })
}

fn manifest_content(ctx: &Context) -> String {
    substitute_context(strip_heredoc(include_str!("./assets/manifest.xml.cr")), ctx)
}

fn java_content(ctx: &Context) -> String {
    substitute_context(strip_heredoc(include_str!("./assets/main.java.cr")), ctx)
}

fn resource_content(ctx: &Context) -> String {
    substitute_context(strip_heredoc(include_str!("./assets/resource.xml.cr")), ctx)
}

fn makefile_content(ctx: &Context) -> String {
    substitute_context(include_str!("./assets/Makefile"), ctx)
}

fn write_utf8_file(contents: &str, path: &Path) -> Result<(), Error> {
    let mut f: File = try!(File::create(path).context(path));
    try!(f.write(contents.as_bytes()).context(PathToWriteTo(path)));
    Ok(())
}

pub const CONTEXT_FILENAME: &'static str = "anders.json";

pub fn generate_application_scaffolding(ctx: &Context) -> Result<(), Error> {
    try!(ctx.verify());
    let dir = Path::new(&ctx.project);
    let app_path = |path: &str| dir.join(path);
    if dir.is_dir() {
        return Err(Error::ExistingDirectory(dir.to_owned()));
    }

    let package_dir = app_path(&dotted_package_name_to_package_path(&ctx.package));
    let resource_dir = app_path("res/values");

    for dir_name in &["lib", "obj", "bin"] {
        let dir = app_path(dir_name);
        try!(create_dir_all(&dir).context(dir.as_path()));
    }

    try!(create_dir_all(&package_dir).context(package_dir.as_path()));
    try!(create_dir_all(&resource_dir).context(resource_dir.as_path()));
    try!(write_utf8_file(&manifest_content(ctx), &app_path("AndroidManifest.xml")));
    try!(write_utf8_file(&makefile_content(ctx), &app_path("Makefile")));
    try!(write_utf8_file(&java_content(ctx),
                         Path::new(&format!("{}/{}.java", package_dir.display(), ctx.project))));
    try!(write_utf8_file(&resource_content(ctx),
                         Path::new(&format!("{}/strings.xml", resource_dir.display()))));
    try!(write_utf8_file(&ctx.serialize(), &app_path(CONTEXT_FILENAME)));
    Ok(())
}


#[test]
fn test_dotted_package_name_to_package_path() {
    assert_eq!(dotted_package_name_to_package_path("hello.wonderful.world"),
               "src/hello/wonderful/world");
}
