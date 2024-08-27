use clap::parser::ValuesRef;
use glob::glob;
use id3::{Content, ErrorKind, Frame, Tag, TagLike, Version};
use prettytable::{row, Table};
use std::{collections::BTreeSet, io::Write, path::PathBuf};
use tempfile::NamedTempFile;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
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
}

pub fn show_tags(paths: ValuesRef<String>) -> Result<(), CommandError> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;
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
        let file_name: &str = path.file_name().unwrap().to_str().unwrap();
        let composer: &str = match tag.get("TCOM") {
            Some(frame) => frame.content().text().unwrap(),
            None => "",
        };

        table.add_row(row![
            file_name,
            tag.title().unwrap_or(""),
            tag.album().unwrap_or(""),
            tag.artist().unwrap_or(""),
            tag.album_artist().unwrap_or(""),
            composer,
            tag.disc().unwrap_or(0u32),
            tag.track().unwrap_or(0u32),
        ]);
    }
    table.printstd();
    Ok(())
}

pub fn number_files(paths: ValuesRef<String>, start: u32) -> Result<(), CommandError> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for (path, i) in paths.iter().zip(start..) {
        let mut tag: Tag = read_tag(path)?;
        tag.set_track(i);
        if let Err(err) = tag.write_to_path(path, id3::Version::Id3v23) {
            return Err(CommandError::Id3Error(err));
        }
    }
    Ok(())
}

pub fn number_chapters(
    naming_scheme: &str,
    paths: ValuesRef<String>,
    start: i32,
) -> Result<(), CommandError> {
    if !naming_scheme.contains("%n") {
        return Err(CommandError::NoFormatSpecifierError("%n".to_string()));
    }
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for (path, i) in paths.iter().zip(start..) {
        let chapter_name = naming_scheme.replace("%n", &i.to_string());
        write_frame(path, "TIT2", &chapter_name)?;
    }
    Ok(())
}

pub fn change_title(title: &str, paths: ValuesRef<String>) -> Result<(), CommandError> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for path in &paths {
        write_frame(path, "TIT2", title)?;
    }
    Ok(())
}

pub fn change_author(author: &str, paths: ValuesRef<String>) -> Result<(), CommandError> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for path in &paths {
        write_frame(path, "TPE1", author)?;
    }
    Ok(())
}

pub fn change_narrator(narrator: &str, paths: ValuesRef<String>) -> Result<(), CommandError> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for path in &paths {
        write_frame(path, "TCOM", narrator)?;
    }
    Ok(())
}

pub fn change_tag(
    frame_id: &str,
    new_text: &str,
    paths: ValuesRef<String>,
) -> Result<(), CommandError> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for path in &paths {
        write_frame(path, frame_id, new_text)?;
    }
    Ok(())
}

pub fn combine_files(
    paths: ValuesRef<String>,
    output: &str,
    bitrate: u32,
    title: &str,
    author: &str,
) -> Result<(), CommandError> {
    let paths = expand_wildcards(paths)?;
    let file_tmp_buf: String = paths
        .iter()
        .map(|path| format!("file '{}'", path.to_str().unwrap()))
        .collect::<Vec<String>>()
        .join("\n");
    let mut file_tmp = NamedTempFile::new()?;
    file_tmp.write_all(file_tmp_buf.as_bytes())?;

    Ok(())
}

pub fn generate_metadata(paths: BTreeSet<PathBuf>, title: &str, author: &str) {
    let mut ffmetadata = format!("
;FFMETADATA
title={title}
artist={author}
genre=AudioBook
");
    

}

fn write_frame(path: &PathBuf, frame_id: &str, new_text: &str) -> Result<(), CommandError> {
    let mut tag: Tag = read_tag(&path)?;
    let frame = Frame::with_content(frame_id, Content::Text(new_text.to_string()));
    tag.add_frame(frame);
    if let Err(err) = tag.write_to_path(path, Version::Id3v23) {
        return Err(CommandError::Id3Error(err));
    }
    Ok(())
}

fn expand_wildcards(raw_paths: ValuesRef<String>) -> Result<BTreeSet<PathBuf>, CommandError> {
    let mut parsed_paths: BTreeSet<PathBuf> = BTreeSet::new();

    for raw_path in raw_paths {
        match glob(&raw_path) {
            Ok(globs) => {
                for glob_path in globs {
                    parsed_paths.insert(glob_path.unwrap());
                }
            }
            Err(glob_error) => return Err(CommandError::GlobError(glob_error)),
        }
    }
    if parsed_paths.len() == 0 {
        return Err(CommandError::NoFilesFountError);
    }
    Ok(parsed_paths)
}

fn read_tag(path: &PathBuf) -> Result<Tag, CommandError> {
    match Tag::read_from_path(path) {
        Ok(tag) => Ok(tag),
        Err(id3::Error {
            kind: ErrorKind::NoTag,
            ..
        }) => Ok(Tag::new()),
        Err(err) => {
            return Err(CommandError::Id3Error(err));
        }
    }
}
