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
                .num_args(1..)
                .value_parser(value_parser!(PathBuf))
            ))

        .subcommand(
            Command::new("number-files")
            .about("Update the track number tag of each file with a sequential number, starting from the specified value.")
            .arg(
                Arg::new("paths")
                .required(true)
                .num_args(1..)
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
        .subcommand(
            Command::new("number-chapters")
            .about("Update the title tag of each file with a name based on a naming scheme, replacing '%n' with a sequential number, starting from specified value.")
            .arg(
                Arg::new("naming-scheme")
                .required(true)
            )
            .arg(
                Arg::new("paths")
                .required(true)
                .num_args(1..)
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
        .subcommand(
            Command::new("change-title")
            .about("Change the title tag of each specified file to the given title.")
            .arg(
                Arg::new("title")
                .required(true)
            )
        )
    .get_matches();

    matches
}
