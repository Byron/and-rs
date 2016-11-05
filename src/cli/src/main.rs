#[macro_use]
extern crate quick_error;
extern crate clap;
extern crate anders;

use quick_error::ResultExt;
use std::collections::HashMap;
use std::process::exit;
use std::fs::File;
use std::io::{self, Write, stderr};
use clap::{App, Arg, SubCommand, ArgMatches};
use anders::scaffolding::{generate_application_scaffolding, CONTEXT_FILENAME};
use anders::compile::compile_application;
use anders::package::package_application;
use std::error::Error as StdError;
use std::fmt::{self, Formatter, Display};

use std::path::{Path, PathBuf};

struct WithCauses<'a>(&'a StdError);

impl<'a> Display for WithCauses<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        try!(write!(fmt, "ERROR: {}", self.0));
        let mut cursor = self.0;
        while let Some(err) = cursor.cause() {
            try!(write!(fmt, "\ncaused by: \n{}", err));
            cursor = err;
        }
        try!(write!(fmt, "\n"));
        Ok(())
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ContextReadingIo(p: PathBuf, err: io::Error) {
            description("The context file could not be read")
            display("Failed to read context from '{}', use -c <path> to specify it", p.display())
            context(p: & 'a Path, err: io::Error) -> (p.to_path_buf(), err)
            cause(err)
        }
        ContextSchema(p: PathBuf, err: anders::ContextDeserializationError) {
            description("The context file had an invalid format")
            display("Failed to interpret schema of context at '{}'", p.display())
            cause(err)
        }
    }
}

fn ok_or_exit<T, E>(res: Result<T, E>) -> T
    where E: std::error::Error
{
    match res {
        Ok(res) => res,
        Err(err) => {
            write!(stderr(), "{}\n", WithCauses(&err)).ok();
            exit(3);
        }
    }
}

fn context_from<'a>(args: &'a ArgMatches<'a>) -> Result<(PathBuf, anders::Context), Error> {
    let (context_path, context_dir) = {
        let path = PathBuf::from(args.value_of("context").expect("context to be mandatory"));
        if path.is_dir() {
            (path.join(CONTEXT_FILENAME), path)
        } else {
            let dir = path.parent().unwrap_or_else(|| Path::new(".")).to_owned();
            (path, dir)
        }
    };
    let mut file = try!(File::open(&context_path).context(context_path.as_path()));
    anders::Context::deserialize(&mut file)
        .map(|ctx| (context_dir, ctx))
        .map_err(|err| Error::ContextSchema(context_path.to_owned(), err))
}

fn build_tasks() -> HashMap<String, anders::Task> {
    let mut map = HashMap::new();
    for task_name in &["compile", "package"] {
       map.insert(String::from(*task_name), anders::Task {
           before: Some(format!("echo before {}", task_name)),
           after: Some(format!("echo after {}", task_name)),
       });
    }
    map
}

fn to_context<'a>(args: &ArgMatches<'a>) -> anders::Context {
    anders::Context {
        project: args.value_of("app-name").expect("app-name to be mandatory").to_owned(),
        package: args.value_of("package").expect("package to be mandatory").to_owned(),
        target: args.value_of("target").expect("target to be mandatory").to_owned(),
        tasks: build_tasks()
    }
}

fn new_app<'a, 'b>() -> App<'a, 'b> {
    fn context<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("context")
            .short("c")
            .long("context")
            .required(false)
            .takes_value(true)
            .default_value(".")
            .help("path to the file created after executing new, or to the directory \
                       containing it.")
    }
    App::new("anders")
        .version("1.0")
        .author("Sebastian Thiel")
        .about("Comfortable android development from your command-line")
        .subcommand(SubCommand::with_name("new")
            .display_order(0)
            .about("create scaffolding for a new hello-world android app")
            .version("0.1")
            .arg(Arg::with_name("app-name")
                .required(true)
                .index(1)
                .help("name of the android app"))
            .arg(Arg::with_name("package")
                .short("p")
                .long("package")
                .required(true)
                .takes_value(true)
                .help("name of the java package, e.g. com.company.package"))
            .arg(Arg::with_name("target")
                .short("t")
                .long("target")
                .required(true)
                .takes_value(true)
                .help("name of the Android target, e.g. 'android-25' as listed by `android list \
                       target`")))
        .subcommand(SubCommand::with_name("compile")
            .display_order(1)
            .about("compile program files and resources")
            .version("0.1")
            .arg(context()))
        .subcommand(SubCommand::with_name("package")
            .display_order(2)
            .about("package previously compiled artifacts into a package signed with the Android \
                    Debug Key")
            .version("0.1")
            .arg(context()))
}

fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        ("new", Some(args)) => {
            ok_or_exit(generate_application_scaffolding(&to_context(args)));
        }
        (cmd @ "compile", Some(args)) |
        (cmd @ "package", Some(args)) => {
            let (project_root, ctx) = ok_or_exit(context_from(args));
            match cmd {
                "compile" => ok_or_exit(compile_application(&project_root, &ctx)),
                "package" => ok_or_exit(package_application(&project_root, &ctx)),
                _ => unreachable!(),
            }
        }
        _ => {
            println!("{}", matches.usage());
            exit(4);
        }
    }
}

fn main() {
    let matches = new_app().get_matches();
    handle(matches);
}
