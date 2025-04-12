use std::path::PathBuf;
use audiobook_tagger::{
    change_author, change_narrator, change_tag, change_title, extract_chapters, combine_files,
    example_toml, number_chapters, number_files, show_chapters, show_tags, embed_chapters,
};
use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::ShowTags { paths } => show_tags(paths),
        Commands::NumberFiles { paths, start } => number_files(paths, start),
        Commands::NumberFileTitles {
            naming_scheme,
            paths,
            start,
        } => number_chapters(&naming_scheme, paths, start),
        Commands::ChangeTitle { title, paths } => change_title(&title, paths),
        Commands::ChangeAuthor { author, paths } => change_author(&author, paths),
        Commands::ChangeNarrator { narrator, paths } => change_narrator(&narrator, paths),
        Commands::ChangeTag { tag, value, paths } => change_tag(&tag, &value, paths),
        Commands::CombineFiles {
            input,
            output,
            bitrate,
            title,
            author,
            ffmpeg_path,
        } => combine_files(input, &output, bitrate, &title, &author, &ffmpeg_path),
        Commands::ShowChapters { path } => show_chapters(&path),
        Commands::ExtractChapters { path, output } => extract_chapters(&path, output),
        Commands::EmbedChapters {
            input,
            toml,
            output,
            ffmpeg_path,
        } => embed_chapters(&input, &output, &toml, &ffmpeg_path),
        Commands::ExampleToml { output } => example_toml(output),
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
        paths: Vec<PathBuf>,
    },

    /// Update the track number tag of each file with a sequential
    /// number.
    NumberFiles {
        #[arg(num_args = 1..)]
        paths: Vec<PathBuf>,
        /// Start value for numbering
        #[arg(long, short, default_value_t = 1)]
        start: u32,
    },

    /// Update the title tag of each file with a name based on a
    /// naming scheme, replacing '%n' with a sequential number
    NumberFileTitles {
        /// String containing format specifier '%n' for the number
        naming_scheme: String,
        #[arg(num_args = 1..)]
        paths: Vec<PathBuf>,
        /// Start value for numbering
        #[arg(long, short, default_value_t = 1)]
        start: u32,
    },

    /// Change the title tag of each specified file to the given title.
    ChangeTitle {
        title: String,
        #[arg(num_args = 1..)]
        paths: Vec<PathBuf>,
    },

    /// Change the author tag of each specified file to the given author name.
    ChangeAuthor {
        author: String,
        #[arg(num_args = 1..)]
        paths: Vec<PathBuf>,
    },

    /// Change the narrator (composer) tag of each specified file to the given name.
    ChangeNarrator {
        narrator: String,
        #[arg(num_args = 1..)]
        paths: Vec<PathBuf>,
    },

    /// Change a specified ID3 tag of each file to the given value.
    ChangeTag {
        /// ID3 tag id
        tag: String,
        /// New tag value
        value: String,
        #[arg(num_args = 1..)]
        paths: Vec<PathBuf>,
    },

    /// Combine multiple audio files into a single file,
    /// with the input files as chapter markers.
    CombineFiles {
        #[arg(num_args = 1..)]
        input: Vec<PathBuf>,
        /// Output path
        #[arg(long, short, default_value = "./output.m4b")]
        output: PathBuf,
        /// Audio bitrate
        #[arg(long, short, default_value_t = 64)]
        bitrate: u32,
        #[arg(long, short, default_value = "Unknown Title")]
        title: String,
        #[arg(long, short, default_value = "Unknown Author")]
        author: String,
        /// Path to ffmpeg
        #[arg(long, short, default_value = "ffmpeg")]
        ffmpeg_path: PathBuf,
    },

    /// Show the embedded chapters in an audiobook file (e.g. m4b or mp4).
    ShowChapters { path: PathBuf },

    /// Reads embedded chapters from audiobook file and outputs TOML to file or stdout.
    ExtractChapters {
        path: PathBuf,
        /// Optional file output path
        #[arg(long, short)]
        output: Option<PathBuf>,
    },

    /// Reads TOML-file with chapters and writes them to an audiobook file.
    EmbedChapters {
        /// Audio input path
        input: PathBuf,
        /// Path to TOML
        toml: PathBuf,
        /// Audio output path
        #[arg(long, short, default_value = "chaptered.m4b")]
        output: PathBuf,
        /// Path to ffmpeg
        #[arg(long, short, default_value = "ffmpeg")]
        ffmpeg_path: PathBuf,
    },

    /// Outputs an example TOML with chapters.
    ExampleToml {
        /// Optional file output path
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
}
