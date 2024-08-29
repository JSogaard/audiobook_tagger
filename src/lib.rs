pub mod chapters;
pub mod helper;

use chapters::ChapterList;
use clap::parser::ValuesRef;
use helper::*;
use id3::{Tag, TagLike};
use mp3_duration::MP3DurationError;
use prettytable::{row, Table};
use std::{collections::BTreeSet, io::Write, path::PathBuf, process::Command};
use tempfile::NamedTempFile;
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
}

pub fn show_tags(paths: ValuesRef<String>) -> Result<(), Error> {
    let paths: BTreeSet<PathBuf> = helper::expand_wildcards(paths)?;
    let mut table = Table::new();
    table.add_row(row![
        b->"File",
        b->"Title",
        b->"Album",
        b->"Author\n(Artist)",
        b->"Album Artist",
        b->"Narrator\n(Composer)",
        b->"Disc",
        b->"Track",
    ]);

    for path in &paths {
        let tag = Tag::read_from_path(&path).unwrap_or(Tag::new());
        let file_name: &str = match path.file_name() {
            Some(file_name) => file_name.to_str().unwrap(),
            None => return Err(Error::NoFilesFountError),
        };
        let composer: &str = match tag.get("TCOM") {
            Some(frame) => frame.content().text().unwrap(),
            None => "",
        };
        let disc: String = match tag.disc() {
            Some(disc) => disc.to_string(),
            None => "".to_string(),
        };
        let track: String = match tag.track() {
            Some(track) => track.to_string(),
            None => "".to_string(),
        };

        table.add_row(row![
            file_name,
            tag.title().unwrap_or(""),
            tag.album().unwrap_or(""),
            tag.artist().unwrap_or(""),
            tag.album_artist().unwrap_or(""),
            composer,
            disc,
            track,
        ]);
    }
    table.printstd();
    Ok(())
}

pub fn number_files(paths: ValuesRef<String>, start: u32) -> Result<(), Error> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for (path, i) in paths.iter().zip(start..) {
        let mut tag: Tag = read_tag(path)?;
        tag.set_track(i);
        if let Err(err) = tag.write_to_path(path, id3::Version::Id3v23) {
            return Err(Error::Id3Error(err));
        }
    }
    Ok(())
}

pub fn number_chapters(
    naming_scheme: &str,
    paths: ValuesRef<String>,
    start: i32,
) -> Result<(), Error> {
    if !naming_scheme.contains("%n") {
        return Err(Error::NoFormatSpecifierError("%n".to_string()));
    }
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for (path, i) in paths.iter().zip(start..) {
        let chapter_name = naming_scheme.replace("%n", &i.to_string());
        write_tag(path, "TIT2", &chapter_name)?;
    }
    Ok(())
}

pub fn change_title(title: &str, paths: ValuesRef<String>) -> Result<(), Error> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for path in &paths {
        write_tag(path, "TIT2", title)?;
    }
    Ok(())
}

pub fn change_author(author: &str, paths: ValuesRef<String>) -> Result<(), Error> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for path in &paths {
        write_tag(path, "TPE1", author)?;
    }
    Ok(())
}

pub fn change_narrator(narrator: &str, paths: ValuesRef<String>) -> Result<(), Error> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for path in &paths {
        write_tag(path, "TCOM", narrator)?;
    }
    Ok(())
}

pub fn change_tag(frame_id: &str, new_text: &str, paths: ValuesRef<String>) -> Result<(), Error> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for path in &paths {
        write_tag(path, frame_id, new_text)?;
    }
    Ok(())
}

pub fn combine_files(
    paths: ValuesRef<String>,
    output: &str,
    bitrate: u32,
    title: &str,
    author: &str,
) -> Result<(), Error> {
    let paths = expand_wildcards(paths)?;
    let file_tmp_buf: String = paths
        .iter()
        .map(|path| format!("file '{}'", path.to_str().unwrap()))
        .collect::<Vec<String>>()
        .join("\n");
    let mut files_tmp = NamedTempFile::new()?;
    files_tmp.write_all(file_tmp_buf.as_bytes())?;

    let mut ffmetadata_tmp = NamedTempFile::new()?;
    // let ffmetadata: String = generate_metadata(&paths, title, author)?;
    let chapter_list =
        ChapterList::from_path_set(paths.iter(), title.to_string(), author.to_string())?;
    let ffmetadata = chapter_list.ffmetadata();
    ffmetadata_tmp.write_all(ffmetadata.as_bytes())?;

    let bitrate = format!("{bitrate}k");

    let arguments: Vec<&str> = vec![
        // "-v",
        // "info",
        "-f",
        "concat",
        "-safe",
        "0",
        "-i",
        files_tmp.path().to_str().unwrap(),
        "-i",
        ffmetadata_tmp.path().to_str().unwrap(),
        "-map_metadata",
        "1",
        "-c:a",
        "aac",
        "-b:a",
        &bitrate,
        output,
    ];
    let status = Command::new("ffmpeg").args(arguments).status()?;
    match status.code() {
        Some(0) => {
            println!("Finished");
            Ok(())
        }
        Some(code) => Err(Error::FfmpegError(code)),
        None => Err(Error::FfmpegError(1)),
    }
}
