use crate::errors::*;
use chapters::ChapterList;
use clap::parser::ValuesRef;
use helper::*;
use id3::{Tag, TagLike};
use prettytable::{row, Table};
use std::{
    collections::BTreeSet,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
use tempfile::NamedTempFile;

use crate::{chapters, helper};

pub fn show_tags(paths: ValuesRef<String>) -> Result<()> {
    let paths: BTreeSet<PathBuf> = helper::expand_wildcards(paths)?;
    let mut table = Table::new();
    table.set_titles(row![
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
            Some(file_name) => &file_name.to_string_lossy(),
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

pub fn number_files(paths: ValuesRef<String>, start: u32) -> Result<()> {
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

pub fn number_chapters(naming_scheme: &str, paths: ValuesRef<String>, start: i32) -> Result<()> {
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

pub fn change_title(title: &str, paths: ValuesRef<String>) -> Result<()> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for path in &paths {
        write_tag(path, "TIT2", title)?;
    }
    Ok(())
}

pub fn change_author(author: &str, paths: ValuesRef<String>) -> Result<()> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for path in &paths {
        write_tag(path, "TPE1", author)?;
    }
    Ok(())
}

pub fn change_narrator(narrator: &str, paths: ValuesRef<String>) -> Result<()> {
    let paths: BTreeSet<PathBuf> = expand_wildcards(paths)?;

    for path in &paths {
        write_tag(path, "TCOM", narrator)?;
    }
    Ok(())
}

pub fn change_tag(frame_id: &str, new_text: &str, paths: ValuesRef<String>) -> Result<()> {
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
    ffmpeg_path: &str,
) -> Result<()> {
    let paths = expand_wildcards(paths)?;
    let file_tmp_buf: String = paths
        .iter()
        .map(|path| format!("file '{}'", path.to_string_lossy()))
        .collect::<Vec<String>>()
        .join("\n");
    let mut files_tmp = NamedTempFile::new()?;
    files_tmp.write_all(file_tmp_buf.as_bytes())?;
    let files_tmp_path = files_tmp.path().to_string_lossy();

    let mut ffmetadata_tmp = NamedTempFile::new()?;
    // let ffmetadata: String = generate_metadata(&paths, title, author)?;
    let chapter_list = ChapterList::from_path_set(paths, title.to_string(), author.to_string())?;
    let ffmetadata = chapter_list.ffmetadata();
    ffmetadata_tmp.write_all(ffmetadata.as_bytes())?;
    let ffmetadata_tmp_path = ffmetadata_tmp.path().to_string_lossy();

    let bitrate = format!("{bitrate}k");

    let arguments = [
        "-f",
        "concat",
        "-safe",
        "0",
        "-i",
        &files_tmp_path,
        "-i",
        &ffmetadata_tmp_path,
        "-map_metadata",
        "1",
        "-c:a",
        "aac",
        "-b:a",
        &bitrate,
        output,
    ];
    run_ffmpeg(ffmpeg_path, arguments)
}

pub fn show_chapters(path: &str) -> Result<()> {
    let chapter_list = ChapterList::from_chaptered_file(path)?;
    println!("{}", chapter_list);
    Ok(())
}

pub fn chapters_to_toml(path: &str) -> Result<()> {
    let chapter_toml = ChapterList::from_chaptered_file(path)?.toml()?;
    print!("{}", &chapter_toml);

    Ok(())
}

pub fn toml_to_chapters(
    path: &str,
    output: &str,
    toml_path: &str,
    ffmpeg_path: &str,
) -> Result<()> {
    let mut toml = String::new();
    File::open(toml_path)?.read_to_string(&mut toml)?;
    ChapterList::from_toml(&toml)?.write_to_file(path, output, ffmpeg_path)?;

    Ok(())
}
