use mp3_duration::MP3DurationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An error occured while reading or writing to file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("An error occured while reading or writing to an id3-tag: {0}")]
    Id3Error(#[from] id3::Error),

    #[error("No files matched the provided pattern")]
    NoFilesFountError,

    #[error("An error occured while parsing path wild cards: {0}")]
    GlobError(#[from] glob::PatternError),

    #[error("The pattern did not contain the correct format specifier: {0}")]
    NoFormatSpecifierError(String),

    #[error("An error occured while reading MP3 file duration: {0}")]
    NoDurationError(#[from] MP3DurationError),

    #[error("ffmpeg encountered an error: {0}")]
    FfmpegError(i32),

    #[error("Could not find ffmpeg executable: {0}")]
    FfmpegNotFoundError(String),

    #[error("Could not find ffprobe. It is automatically installed with ffmpeg")]
    FfprobeNotFoundError(),

    #[error("An error occured while reading the chapters of the audio file")]
    ChapterReadError,

    #[error("Failed write to TOML data: {0}")]
    TomlSerializationError(#[from] toml::ser::Error),

    #[error("Failed read from TOML data: {0}")]
    TomlDeserializationError(#[from] toml::de::Error),


    #[error("Filed to read from stdin")]
    StdinError
}

pub type Result<T> = std::result::Result<T, Error>;
