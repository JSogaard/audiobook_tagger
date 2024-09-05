use audiobook_tagger::{
    change_author, change_narrator, change_tag, change_title, chapters_to_toml, combine_files, example_toml, number_chapters, number_files, show_chapters, show_tags, toml_to_chapters
};
use clap::{command, parser::ValuesRef, value_parser, Arg, ArgMatches, Command};

fn main() -> anyhow::Result<()> {
    let matches: ArgMatches = cli();

    if let Some((subcommand, args)) = matches.subcommand() {
        match subcommand {
            "show-tags" => show_tags(args.get_many::<String>("paths").unwrap())?,
            "number-files" => {
                let paths: ValuesRef<String> = args.get_many("paths").unwrap();
                let start: &u32 = args.get_one::<u32>("start").unwrap();
                number_files(paths, *start)?;
            }
            "number-file-titles" => {
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
            "change-tag" => {
                let frame_id: &String = args.get_one("tag").unwrap();
                let new_text: &String = args.get_one("value").unwrap();
                let paths: ValuesRef<String> = args.get_many("paths").unwrap();
                change_tag(frame_id, new_text, paths)?;
            }
            "combine-files" => {
                let paths: ValuesRef<String> = args.get_many("paths").unwrap();
                let output: &String = args.get_one("output").unwrap();
                let bitrate: &u32 = args.get_one("bitrate").unwrap();
                let title: &String = args.get_one("title").unwrap();
                let author: &String = args.get_one("author").unwrap();
                let ffmpeg_path: &String = args.get_one("ffmpeg-path").unwrap();
                combine_files(paths, output, *bitrate, title, author, ffmpeg_path)?;
            }
            "show-chapters" => {
                let path: &String = args.get_one("path").unwrap();
                show_chapters(path)?;
            }
            "chapters-to-toml" => {
                let path: &String = args.get_one("path").unwrap();
                chapters_to_toml(path)?;
            }
            "toml-to-chapters" => {
                let path: &String = args.get_one("path").unwrap();
                let toml: &String = args.get_one("toml").unwrap();
                let output: &String = args.get_one("output").unwrap();
                let ffmpeg_path: &String = args.get_one("ffmpeg-path").unwrap();
                toml_to_chapters(path, output, toml, &ffmpeg_path)?;
            }
            "example-toml" => example_toml(),
            _ => {}
        }
    }
    Ok(())
}

fn cli() -> ArgMatches {
    let matches = command!()
        .subcommand_required(true)
        .about(
            "Tool to prepare audiobook files by changing metadata and \
            combining multiple mp3 files into one m4b",
        )
        .subcommand(
            Command::new("show-tags")
                .about("Show common ID3 tags from files.")
                .arg(
                    Arg::new("paths").required(true).num_args(1..), // .value_parser(value_parser!(PathBuf))
                ),
        )
        .subcommand(
            Command::new("number-files")
                .about(
                    "Update the track number tag of each file with a sequential \
                    number, starting from the specified value. The starting \
                    number must be positiv or zero",
                )
                .arg(
                    Arg::new("paths").required(true).num_args(1..), // .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("start")
                        .long("start")
                        .short('s')
                        .value_parser(value_parser!(u32))
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("number-file-titles")
                .about(
                    "Update the title tag of each file with a name based on a \
                    naming scheme, replacing '%n' with a sequential number, \
                    starting from specified value.",
                )
                .arg(Arg::new("naming-scheme").required(true))
                .arg(
                    Arg::new("paths").required(true).num_args(1..), // .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("start")
                        .long("start")
                        .short('s')
                        .value_parser(value_parser!(i32))
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("change-title")
                .about("Change the title tag of each specified file to the given title.")
                .arg(Arg::new("title").required(true))
                .arg(
                    Arg::new("paths").required(true).num_args(1..), // .value_parser(value_parser!(PathBuf))
                ),
        )
        .subcommand(
            Command::new("change-author")
                .about(
                    "Change the author tag of each specified \
                    file to the given author name.",
                )
                .arg(Arg::new("author").required(true))
                .arg(
                    Arg::new("paths").required(true).num_args(1..), // .value_parser(value_parser!(PathBuf))
                ),
        )
        .subcommand(
            Command::new("change-narrator")
                .about(
                    "Change the narrator (composer) tag of each specified \
                    file to the given narrator name.",
                )
                .arg(Arg::new("narrator").required(true))
                .arg(
                    Arg::new("paths").required(true).num_args(1..), // .value_parser(value_parser!(PathBuf))
                ),
        )
        .subcommand(
            Command::new("change-tag")
                .about("Change a specified tag of each file to the given value.")
                .arg(Arg::new("tag").required(true))
                .arg(Arg::new("value").required(true))
                .arg(
                    Arg::new("paths").required(true).num_args(1..), // .value_parser(value_parser!(PathBuf))
                ),
        )
        .subcommand(
            Command::new("combine-files")
                .about(
                    "Combine multiple audio files into a single file, \
                    with the input files as chapter markers.",
                )
                .arg(
                    Arg::new("paths").required(true).num_args(1..), // .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .default_value("./output.m4b"), // .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("bitrate")
                        .long("bitrate")
                        .short('b')
                        .default_value("64")
                        .value_parser(value_parser!(u32)),
                )
                .arg(
                    Arg::new("title")
                        .long("title")
                        .short('t')
                        .default_value("Unknown title"),
                )
                .arg(
                    Arg::new("author")
                        .long("author")
                        .short('a')
                        .default_value("Unknown author"),
                )
                .arg(
                    Arg::new("ffmpeg-path")
                        .long("with-ffmpeg")
                        .short('w')
                        .default_value("ffmpeg"),
                ),
        )
        .subcommand(
            Command::new("show-chapters")
                .about("Show the embedded chapters in an audiobook file (e.g. m4b or mp4)")
                .arg(Arg::new("path").required(true)),
        )
        .subcommand(
            Command::new("chapters-to-toml")
                .about(
                    "Reads embedded chapters from audiobook file and \
                    outputs them to stdout as TOML",
                )
                .arg(Arg::new("path").required(true)),
        )
        .subcommand(
            Command::new("toml-to-chapters")
                .about("Reads TOML-file with chapters and writes them to an audiobook file")
                .arg(Arg::new("path").required(true))
                .arg(Arg::new("toml").required(true))
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .default_value("chaptered.m4b"),
                )
                .arg(
                    Arg::new("ffmpeg-path")
                        .long("with-ffmpeg")
                        .short('w')
                        .default_value("ffmpeg"),
                ),
        )
        .subcommand(Command::new("example-toml").about(
            "Outputs an example TOML to stdout as a template for creating \
                chapters for an audiobook file",
        ))
        .get_matches();

    matches
}
