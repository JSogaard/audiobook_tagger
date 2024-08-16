use clap::{command, value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;

fn main() {
    let matches = cli();
}

fn cli() -> ArgMatches {
    let matches = command!().subcommand_required(true)
    .subcommand(
        Command::new("show-tags")
        .about("Show common ID3 tags from files.")
        .arg(
            Arg::new("paths")
            .required(true)
            .value_parser(value_parser!(PathBuf))
        ))

    .subcommand(
        Command::new("number-files")
        .about("Update the track number tag of each file with a sequential number, starting from the specified value.")
        .arg(
            Arg::new("paths")
            .required(true)
            .value_parser(value_parser!(PathBuf))
        )
        .arg(
            Arg::new("start")
            .long("start")
            .short('s')
            .value_parser(value_parser!(i32))
            .default_value("1")
        )
    )
    .get_matches();

    matches
    
}
