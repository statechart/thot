extern crate clap;
extern crate serde_json;
extern crate thot;

use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::prelude::*;
use thot::ast::core::Core;
use thot::ast::statechart::Statechart;

fn main() {
    let matches = App::new("Statechart CLI")
        .version("1.0")
        // .arg(Arg::with_name("config")
        //      .short("c")
        //      .long("config")
        //      .value_name("FILE")
        //      .help("Sets a custom config file")
        //      .takes_value(true))
        // .arg(Arg::with_name("INPUT")
        //      .help("Sets the input file to use")
        //      .required(true)
        //      .index(1))
        // .arg(Arg::with_name("v")
        //      .short("v")
        //      .multiple(true)
        //      .help("Sets the level of verbosity"))
        .subcommand(
            SubCommand::with_name("compile")
                .about("Compile statecharts")
                .version("1.0")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .required(true)
                )
                .arg(
                    Arg::with_name("debug")
                      .short("d")
                      .help("print debug information verbosely")
                )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("compile") {
        let input = matches.value_of("INPUT").unwrap();
        let mut f = File::open(input).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let statechart: Statechart = serde_json::from_str(&contents).unwrap();

        let core: Result<Core, _> = statechart.into();

        let out = serde_json::to_string_pretty(&core.unwrap()).unwrap();

        println!("{}", out);
    }
}
