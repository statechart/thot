extern crate clap;
extern crate serde_json;
extern crate thot_core;

use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::prelude::*;
use thot_core::ast::core::Core;
use thot_core::ast::microstep::Microstep;
use thot_core::ast::statechart::Statechart;

fn main() {
    let matches = App::new("Statechart CLI")
        .version("1.0")
        .subcommand(
            SubCommand::with_name("compile")
                .about("Compile statecharts")
                .version("1.0")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .required(true),
                )
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
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
        let microstep: Result<Microstep, _> = core.unwrap().into();

        let out = serde_json::to_string(&microstep.unwrap()).unwrap();

        println!("{}", out);
    }
}
