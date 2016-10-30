extern crate clap;
extern crate anders;

use std::process::exit;
use std::io::{Write, stderr};
use clap::{App, Arg, SubCommand, ArgMatches};

fn die_with<E>(err: E)
    where E: std::error::Error
{
    write!(stderr(), "{}\n", err).ok();
    exit(3);
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
    if let Err(err) = match matches.subcommand() {
        ("new", Some(args)) => anders::generate_application_scaffolding(&to_context(args)),
        ("compile", Some(args)) => Ok(()),
        _ => {
            println!("{}", matches.usage());
            exit(4);
        }
    } {
        die_with(err);
    }
}

fn main() {
    let matches = new_app().get_matches();
    handle(matches);
}
