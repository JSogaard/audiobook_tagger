use audiobook_tagger::{change_author, change_narrator, change_tag, change_title, chapters_to_toml, combine_files, example_toml, number_chapters, number_files, show_chapters, show_tags, toml_to_chapters};
use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::ShowTags { paths } => show_tags(paths),
        Commands::NumberFiles { paths, start } => number_files(paths, start),
        Commands::NumberFileTitles { naming_scheme, paths, start } => {
            number_chapters(&naming_scheme, paths, start)
        },
        Commands::ChangeTitle { title, paths } => change_title(&title, paths),
        Commands::ChangeAuthor { author, paths } => change_author(&author, paths),
        Commands::ChangeNarrator { narrator, paths } => change_narrator(&narrator, paths),
        Commands::ChangeTag { tag, value, paths } => change_tag(&tag, &value, paths),
        Commands::CombineFiles { input, output, bitrate, title, author, ffmpeg_path } => {
            combine_files(input, &output, bitrate, &title, &author, &ffmpeg_path)
        },
        Commands::ShowChapters { path } => show_chapters(&path),
        Commands::ChaptersToToml { path, output } => chapters_to_toml(&path, output),
        Commands::TomlToChapters { input, toml, output, ffmpeg_path } => {
            toml_to_chapters(&input, &output, &toml, &ffmpeg_path)
        },
        Commands::ExampleToml {output} => {
            example_toml(output)
        },
    };

    if let Err(e) = result {
        eprintln!("Error:\n{}", e);
    }
}

#[derive(Parser)]
#[command(name = "Audiobook Tagger")]
#[command(about = "Tool to prepare audiobook files by changing metadata and \
            combining multiple mp3 files into one m4b")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show common ID3 tags from files.
    ShowTags {
        #[arg(num_args = 1..)]
        paths: Vec<String>
    },

    /// Update the track number tag of each file with a sequential
    /// number.
    NumberFiles {
        #[arg(num_args = 1..)]
        paths: Vec<String>,
        /// Start value for numbering
        #[arg(long, short, default_value_t = 1)]
        start: u32,
    },

    /// Change the title tag of each specified file to the given title.
    NumberFileTitles {
        /// String containing format specifier '%n' for the number
        naming_scheme: String,
        #[arg(num_args = 1..)]
        paths: Vec<String>,
        /// Start value for numbering
        #[arg(long, short, default_value_t = 1)]
        start: u32,
    },

    /// Change the title tag of each specified file to the given title.
    ChangeTitle {
        title: String,
        #[arg(num_args = 1..)]
        paths: Vec<String>,
    },

    /// Change the author tag of each specified file to the given author name.
    ChangeAuthor {
        author: String,
        #[arg(num_args = 1..)]
        paths: Vec<String>,
    },

    /// Change the narrator (composer) tag of each specified file to the given name.
    ChangeNarrator {
        narrator: String,
        #[arg(num_args = 1..)]
        paths: Vec<String>,
    },

    /// Change a specified ID3 tag of each file to the given value.
    ChangeTag {
        /// ID3 tag id
        tag: String,
        /// New tag value
        value: String,
        #[arg(num_args = 1..)]
        paths: Vec<String>,
    },

    /// Combine multiple audio files into a single file, 
    /// with the input files as chapter markers.
    CombineFiles {
        #[arg(num_args = 1..)]
        input: Vec<String>,
        /// Output path
        #[arg(long, short, default_value="./output.m4b")]
        output: String,
        /// Audio bitrate
        #[arg(long, short, default_value_t = 64)]
        bitrate: u32,
        #[arg(long, short, default_value="Unknown Title")]
        title: String,
        #[arg(long, short, default_value="Unknown Author")]
        author: String,
        /// Path to ffmpeg
        #[arg(long, short, default_value="ffmpeg")]
        ffmpeg_path: String,
    },

    /// Show the embedded chapters in an audiobook file (e.g. m4b or mp4).
    ShowChapters {
        path: String,
    },

    /// Reads embedded chapters from audiobook file and outputs TOML to file or stdout.
    ChaptersToToml {
        path: String,
        /// Optional file output path
        #[arg(long, short)]
        output: Option<String>,
    },

    /// Reads TOML-file with chapters and writes them to an audiobook file.
    TomlToChapters {
        /// Audio input path
        input: String,
        /// Path to TOML
        toml: String,
        /// Audio output path
        #[arg(long, short, default_value = "chaptered.m4b")]
        output: String,
        /// Path to ffmpeg
        #[arg(long, short, default_value = "ffmpeg")]
        ffmpeg_path: String,
    },

    /// Outputs an example TOML to stdout.
    ExampleToml {
        /// Optional file output path
        #[arg(long, short)]
        output: Option<String>,
    },
}

// fn main() -> anyhow::Result<()> {
//     let matches: ArgMatches = cli();

//     if let Some((subcommand, args)) = matches.subcommand() {
//         match subcommand {
//             "show-tags" => show_tags(args.get_many::<String>("paths").unwrap())?,
//             "number-files" => {
//                 let paths: ValuesRef<String> = args.get_many("paths").unwrap();
//                 let start: &u32 = args.get_one::<u32>("start").unwrap();
//                 number_files(paths, *start)?;
//             }
//             "number-file-titles" => {
//                 let naming_scheme: &String = args.get_one("naming-scheme").unwrap();
//                 let paths: ValuesRef<String> = args.get_many("paths").unwrap();
//                 let start: &i32 = args.get_one("start").unwrap();
//                 number_chapters(naming_scheme, paths, *start)?;
//             }
//             "change-title" => {
//                 let title: &String = args.get_one("title").unwrap();
//                 let paths: ValuesRef<String> = args.get_many("paths").unwrap();
//                 change_title(title, paths)?;
//             }
//             "change-author" => {
//                 let author: &String = args.get_one("author").unwrap();
//                 let paths: ValuesRef<String> = args.get_many("paths").unwrap();
//                 change_author(author, paths)?;
//             }
//             "change-narrator" => {
//                 let narrator: &String = args.get_one("narrator").unwrap();
//                 let paths: ValuesRef<String> = args.get_many("paths").unwrap();
//                 change_narrator(narrator, paths)?;
//             }
//             "change-tag" => {
//                 let frame_id: &String = args.get_one("tag").unwrap();
//                 let new_text: &String = args.get_one("value").unwrap();
//                 let paths: ValuesRef<String> = args.get_many("paths").unwrap();
//                 change_tag(frame_id, new_text, paths)?;
//             }
//             "combine-files" => {
//                 let paths: ValuesRef<String> = args.get_many("paths").unwrap();
//                 let output: &String = args.get_one("output").unwrap();
//                 let bitrate: &u32 = args.get_one("bitrate").unwrap();
//                 let title: &String = args.get_one("title").unwrap();
//                 let author: &String = args.get_one("author").unwrap();
//                 let ffmpeg_path: &String = args.get_one("ffmpeg-path").unwrap();
//                 combine_files(paths, output, *bitrate, title, author, ffmpeg_path)?;
//             }
//             "show-chapters" => {
//                 let path: &String = args.get_one("path").unwrap();
//                 show_chapters(path)?;
//             }
//             "chapters-to-toml" => {
//                 let path: &String = args.get_one("path").unwrap();
//                 chapters_to_toml(path)?;
//             }
//             "toml-to-chapters" => {
//                 let path: &String = args.get_one("path").unwrap();
//                 let toml: &String = args.get_one("toml").unwrap();
//                 let output: &String = args.get_one("output").unwrap();
//                 let ffmpeg_path: &String = args.get_one("ffmpeg-path").unwrap();
//                 toml_to_chapters(path, output, toml, ffmpeg_path)?;
//             }
//             "example-toml" => example_toml(),
//             _ => {}
//         }
//     }
//     Ok(())
// }