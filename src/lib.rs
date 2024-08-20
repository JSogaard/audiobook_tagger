use clap::parser::ValuesRef;
use glob::glob;
use id3::{Error, ErrorKind, Tag, TagLike};
use prettytable::{row, Table};
use std::{collections::BTreeSet, path::PathBuf};

pub fn show_tags(paths: ValuesRef<String>) {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths);
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
}

pub fn number_files(paths: ValuesRef<String>, start: u32) {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths);

    for (path, i) in paths.iter().zip(start..) {
        let mut tag = match Tag::read_from_path(path) {
            Ok(tag) => tag,
            Err(Error {
                kind: ErrorKind::NoTag,
                ..
            }) => Tag::new(),
            Err(err) => {
                eprintln!("Failed to read tag from path: {:?}\nError: {}", path, err);
                panic!();
            }
        };
        tag.set_track(i);
        let _ = tag.write_to_path(path, id3::Version::Id3v23);
    }
}

pub fn expand_wildcards(raw_paths: ValuesRef<String>) -> BTreeSet<PathBuf> {
    let mut parsed_paths = BTreeSet::new();

    for raw_path in raw_paths {
        if let Ok(globs) = glob(&raw_path) {
            for glob_path in globs {
                parsed_paths.insert(glob_path.unwrap());
            }
        } else {
            panic!("A provided path was invalid");
        }
    }
    parsed_paths
}
