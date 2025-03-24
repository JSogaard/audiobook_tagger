use clap::{Parser, Subcommand};

fn main() {
    let _cli = Cli::parse();
}

#[derive(Parser)]
#[command(name = "Audiobook Takker")]
#[command(about = "Tool to prepare audiobook files by changing metadata and \
            combining multiple mp3 files into one m4b")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show common ID3 tags from files.
    ShowTags {
        #[arg(num_args = 1..)]
        paths: String
    },

    /// Update the track number tag of each file with a sequential
    /// number, starting from specified value.
    NumberFiles {
        #[arg(num_args = 1..)]
        paths: String,
        #[arg(long, short, default_value_t = 1)]
        start: u32,
    },

    /// Change the title tag of each specified file to the given title.
    NumberFileTitles {
        naming_scheme: String,
        #[arg(num_args = 1..)]
        paths: Vec<String>,
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

    /// Tool to prepare audiobook files by changing metadata and 
    /// combining multiple mp3 files into one m4b.
    ChangeNarrator {
        narrator: String,
        #[arg(num_args = 1..)]
        paths: Vec<String>,
    },

    /// Change a specified tag of each file to the given value.
    ChangeTag {
        tag: String,
        value: String,
        #[arg(num_args = 1..)]
        paths: Vec<String>,
    },

    /// Combine multiple audio files into a single file, 
    /// with the input files as chapter markers.
    CombineFiles {
        #[arg(num_args = 1..)]
        paths: Vec<String>,
        #[arg(long, short, default_value="./output.m4b")]
        output: String,
        #[arg(long, short, default_value_t = 64)]
        bitrate: u32,
        #[arg(long, short, default_value="Unknown Title")]
        title: String,
        #[arg(long, short, default_value="Unknown Author")]
        author: String,
        #[arg(long, short, default_value="ffmpeg")]
        ffmpeg_path: String,
    },

    /// Show the embedded chapters in an audiobook file (e.g. m4b or mp4).
    ShowChapters {
        path: String,
    },

    /// Reads embedded chapters from audiobook file and outputs them to stdout as TOML.
    ChaptersToToml {
        path: String,
    },

    /// Reads TOML-file with chapters and writes them to an audiobook file.
    TomlToChapters {
        path: String,
        toml: String,
        #[arg(long, short, default_value = "chaptered.m4b")]
        output: String,
        #[arg(long, short, default_value = "ffmpeg")]
        ffmpeg_path: String,
    },

    /// Outputs an example TOML to stdout as a template for creating chapters for an audiobook file.
    ExampleToml,
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