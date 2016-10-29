extern crate clap;
extern crate anders;

use std::process::exit;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("anders")
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
                .help("name of the ")))
        .get_matches();

    match matches.subcommand() {
        ("new", Some(args)) => {

        }
        _ => {
            println!("{}", matches.usage());
            exit(4);
        }
    }
}
