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
            "package" => ctx.package_path.to_owned(),
            "project" => ctx.application_name.to_owned(),
            x => panic!("handle unknown variable: {}", x),
        }
    })
}

fn manifest_content(ctx: &Context) -> String {
    substitute_context(strip_heredoc(include_str!("./assets/manifest.cr")), ctx)
}

fn java_content(ctx: &Context) -> String {
    substitute_context(strip_heredoc(include_str!("./assets/main.cr")), ctx)
}

fn resource_content(ctx: &Context) -> String {
    substitute_context(strip_heredoc(include_str!("./assets/resource.cr")), ctx)
}

fn serialize(ctx: &Context) -> String {
   "tbd".to_owned()
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
    let resource_dir = app_path("res/values");

    try!(create_dir_all(&package_dir).context(package_dir.as_path()));
    try!(create_dir_all(&resource_dir).context(resource_dir.as_path()));
    try!(write_utf8_file(&manifest_content(ctx), &app_path("AndroidManifest.xml")));
    try!(write_utf8_file(&java_content(ctx),
                         Path::new(&format!("{}/{}.java",
                                            package_dir.display(),
                                            ctx.application_name))));
    try!(write_utf8_file(&resource_content(ctx),
                         Path::new(&format!("{}/strings.xml", resource_dir.display()))));
    try!(write_utf8_file(&serialize(ctx), &app_path("anders.json")));
    Ok(())
}


#[test]
fn test_dotted_package_name_to_package_path() {
    assert_eq!(dotted_package_name_to_package_path("hello.wonderful.world"),
               "src/hello/wonderful/world");
}
