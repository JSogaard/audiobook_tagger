use clap::parser::ValuesRef;
use glob::glob;
use id3::{Error, ErrorKind, Tag, TagLike};
use prettytable::{row, Table};
use std::{collections::BTreeSet, io, path::PathBuf};

pub fn show_tags(paths: ValuesRef<String>) -> Result<(), io::Error> {
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

pub fn number_files(paths: ValuesRef<String>, start: u32) -> Result<(), io::Error> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for (path, i) in paths.iter().zip(start..) {
        let mut tag = read_tag(path)?;
        tag.set_track(i);
        if let Err(err) = tag.write_to_path(path, id3::Version::Id3v23) {
            return Err(io::Error::other(err.description));
        }
    }
    Ok(())
}

pub fn number_chapters(naming_scheme: &String, paths: ValuesRef<String>, start: i32) -> Result<(), io::Error> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for (path, i) in paths.iter().zip(start..) {
        let mut tag = read_tag(path)?;
        let chapter_name = naming_scheme.replace("%n", &i.to_string());
        tag.set_title(chapter_name);
        if let Err(err) = tag.write_to_path(path, id3::Version::Id3v23) {
            return Err(io::Error::other(err.description));
        }
    }
    Ok(())
}

fn expand_wildcards(raw_paths: ValuesRef<String>) -> Result<BTreeSet<PathBuf>, io::Error> {
    let mut parsed_paths = BTreeSet::new();

    for raw_path in raw_paths {
        match glob(&raw_path) {
            Ok(globs) => {
                for glob_path in globs {
                    parsed_paths.insert(glob_path.unwrap());
                }
            }
            Err(glob_error) => return Err(io::Error::other(glob_error)),
        }
    }
    if parsed_paths.len() == 0 {
        return Err(io::Error::other("No files matches the provided path argument"));
    }
    Ok(parsed_paths)
}

fn read_tag(path: &PathBuf) -> Result<Tag, io::Error> {
    match Tag::read_from_path(path) {
        Ok(tag) => Ok(tag),
        Err(Error {
            kind: ErrorKind::NoTag,
            ..
        }) => Ok(Tag::new()),
        Err(err) => {
            return Err(io::Error::other(err.description));
        }
}}
