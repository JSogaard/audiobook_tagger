use clap::parser::ValuesRef;
use glob::glob;
use id3::{Content, ErrorKind, Frame, Tag, TagLike, Version};
use prettytable::{row, Table};
use std::{collections::BTreeSet, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("An error occured while reading or writing to file: {0}")]
    IoError(String),
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

    for path in paths {
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
            return Err(CommandError::IoError(err.description));
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

fn write_frame(path: &PathBuf, frame_id: &str, new_text: &str) -> Result<(), CommandError> {
    let mut tag: Tag = read_tag(&path)?;
    let frame = Frame::with_content(frame_id, Content::Text(new_text.to_string()));
    tag.add_frame(frame);
    if let Err(err) = tag.write_to_path(path, Version::Id3v23) {
        return Err(CommandError::IoError(err.description));
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
            Err(glob_error) => return Err(CommandError::IoError(glob_error.msg.to_string())),
        }
    }
    if parsed_paths.len() == 0 {
        return Err(CommandError::IoError(
            "No files matches the provided path argument".to_string(),
        ));
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
            return Err(CommandError::IoError(err.description));
        }
    }
}
