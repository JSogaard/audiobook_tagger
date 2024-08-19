use clap::parser::ValuesRef;
use glob::glob;
use id3::{Tag, TagLike};
use prettytable::{row, Table};
use std::path::PathBuf;

pub fn show_tags(paths: ValuesRef<String>) {
    let paths = expand_wildcards(paths);
    let mut table = Table::new();
    table.add_row(row![
        b->"File",
        b->"Title",
        b->"Album",
        b->"Author",
        b->"Album Artist",
        b->"Disc",
        b->"Track",
    ]);

    for path in paths {
        let tag = Tag::read_from_path(&path).unwrap_or(Tag::new());
        let file_name = path.file_name().unwrap().to_str().unwrap();

        table.add_row(row![
            file_name,
            tag.title().unwrap_or(""),
            tag.album().unwrap_or(""),
            tag.artist().unwrap_or(""),
            tag.album_artist().unwrap_or(""),
            tag.disc().unwrap_or(0u32),
            tag.track().unwrap_or(0u32),
        ]);
    }
    table.printstd();
}

pub fn expand_wildcards(raw_paths: ValuesRef<String>) -> Vec<PathBuf> {
    // TODO Use HashSet instead
    let mut parsed_paths: Vec<PathBuf> = Vec::new();

    // TODO Return errors instead
    for raw_path in raw_paths {
        if let Ok(globs) = glob(&raw_path) {
            for glob_path in globs {
                parsed_paths.push(glob_path.unwrap());
            }
        } else {
            panic!("A provided path was invalid");
        }
    }
    parsed_paths
}
