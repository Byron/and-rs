#[macro_use]
extern crate quick_error;
extern crate clap;
extern crate anders;

use std::process::exit;
use std::io::{self, Write, stderr};
use clap::{App, Arg, SubCommand, ArgMatches};
use anders::scaffolding::generate_application_scaffolding;

use std::path::{Path, PathBuf};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ContextReadingIo(p: PathBuf, err: io::Error) {
            description("The context file could not be read")
            display("Failed to read context from '{}'", p.display())
            context(p: & 'a Path, err: io::Error) -> (p.to_path_buf(), err)
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
            write!(stderr(), "{}\n", err).ok();
            exit(3);
        }
    }
}

fn context_from<'a>(args: &ArgMatches<'a>) -> Result<anders::Context, Error> {
    Ok(anders::Context { ..Default::default() })
}

fn to_context<'a>(args: &ArgMatches<'a>) -> anders::Context {
    anders::Context {
        application_name: args.value_of("app-name").expect("clap to do the checking").to_owned(),
        package_path: args.value_of("package").expect("clap to do the checking").to_owned(),
        ..Default::default()
    }
}

fn new_app<'a, 'b>() -> App<'a, 'b> {
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
                .help("name of the java package, e.g. com.company.package")))
        .subcommand(SubCommand::with_name("compile")
            .display_order(1)
            .about("compile program files and resources")
            .version("0.1")
            .arg(Arg::with_name("context")
                .short("c")
                .long("context")
                .required(false)
                .takes_value(true)
                .default_value("./anders.json")
                .help("path to the file created after executing new.")))
}

fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        ("new", Some(args)) => {
            ok_or_exit(generate_application_scaffolding(&to_context(args)));
        }
        ("compile", Some(args)) => {
            let ctx = ok_or_exit(context_from(args));
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
