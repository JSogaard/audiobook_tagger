use anyhow::Result;
use audiobook_tagger::{
    change_author, change_narrator, change_title, number_chapters, number_files, show_tags,
};
use clap::{command, parser::ValuesRef, value_parser, Arg, ArgMatches, Command};

fn main() -> Result<()> {
    let matches: ArgMatches = cli();

    if let Some((subcommand, args)) = matches.subcommand() {
        match subcommand {
            "show-tags" => show_tags(args.get_many::<String>("paths").unwrap())?,
            "number-files" => {
                let paths: ValuesRef<String> = args.get_many("paths").unwrap();
                let start: &u32 = args.get_one::<u32>("start").unwrap();
                number_files(paths, *start)?;
            }
            "number-chapters" => {
                let naming_scheme: &String = args.get_one("naming-scheme").unwrap();
                let paths: ValuesRef<String> = args.get_many("paths").unwrap();
                let start: &i32 = args.get_one("start").unwrap();
                number_chapters(naming_scheme, paths, *start)?;
            }
            "change-title" => {
                let title: &String = args.get_one("title").unwrap();
                let paths: ValuesRef<String> = args.get_many("paths").unwrap();
                change_title(title, paths)?;
            }
            "change-author" => {
                let author: &String = args.get_one("author").unwrap();
                let paths: ValuesRef<String> = args.get_many("paths").unwrap();
                change_author(author, paths)?;
            }
            "change-narrator" => {
                let narrator: &String = args.get_one("narrator").unwrap();
                let paths: ValuesRef<String> = args.get_many("paths").unwrap();
                change_narrator(narrator, paths)?;
            }
            _ => {}
        }
    }
    Ok(())
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
                // .value_parser(value_parser!(PathBuf))
            ))

        .subcommand(
            Command::new("number-files")
            .about("Update the track number tag of each file with a sequential number, starting from the specified value. The starting number must be positiv or zero")
            .arg(
                Arg::new("paths")
                .required(true)
                .num_args(1..)
                // .value_parser(value_parser!(PathBuf))
            )
            .arg(
                Arg::new("start")
                .long("start")
                .short('s')
                .value_parser(value_parser!(u32))
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
                // .value_parser(value_parser!(PathBuf))
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
            .arg(
                Arg::new("paths")
                .required(true)
                .num_args(1..)
                // .value_parser(value_parser!(PathBuf))
            )
        )
        .subcommand(
            Command::new("change-author")
            .about("Change the author tag of each specified file to the given author name.")
            .arg(
                Arg::new("author")
                .required(true)
            )
            .arg(
                Arg::new("paths")
                .required(true)
                .num_args(1..)
                // .value_parser(value_parser!(PathBuf))
            )
        )
        .subcommand(
            Command::new("change-narrator")
            .about("Change the narrator (composer) tag of each specified file to the given narrator name.")
            .arg(
                Arg::new("narrator")
                .required(true)
            )
            .arg(
                Arg::new("paths")
                .required(true)
                .num_args(1..)
                // .value_parser(value_parser!(PathBuf))
            )
        )
        .subcommand(
            Command::new("change-tag")
            .about("Change a specified tag of each file to the given value.")
            .arg(
                Arg::new("tag")
                .required(true)
            )
            .arg(
                Arg::new("value")
                .required(true)
            )
            .arg(
                Arg::new("paths")
                .required(true)
                .num_args(1..)
                // .value_parser(value_parser!(PathBuf))
            )
        )
        .subcommand(
            Command::new("combine-files")
            .about("Combine multiple audio files into a single file, with the input files as chapter markers.")
            .arg(
                Arg::new("paths")
                .required(true)
                .num_args(1..)
                // .value_parser(value_parser!(PathBuf))
            )
            .arg(
                Arg::new("output")
                .long("output")
                .short('o')
                .default_value("./output.mp4")
                // .value_parser(value_parser!(PathBuf))
            )
            .arg(
                Arg::new("bitrate")
                .long("bitrate")
                .short('b')
                .default_value("64")
                .value_parser(value_parser!(u32))
            )
        )
    .get_matches();

    matches
}
