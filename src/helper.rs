use crate::{Error, Result};
use id3::{Content, Frame, Tag, TagLike, Version};
use std::{collections::BTreeSet, io, path::{Path, PathBuf}, process::Command};

pub fn expand_wildcards(raw_paths: Vec<String>) -> Result<BTreeSet<PathBuf>> {
    let mut parsed_paths: BTreeSet<PathBuf> = BTreeSet::new();

    for raw_path in raw_paths {
        match glob::glob(&raw_path) {
            Ok(globs) => {
                for glob_path in globs {
                    parsed_paths.insert(glob_path.unwrap().canonicalize()?);
                }
            }
            Err(glob_error) => return Err(Error::GlobError(glob_error)),
        }
    }
    if parsed_paths.is_empty() {
        return Err(Error::NoFilesFountError);
    }
    Ok(parsed_paths)
}

pub fn write_tag(path: &PathBuf, frame_id: &str, new_text: &str) -> Result<()> {
    let mut tag: Tag = read_tag(path)?;
    let frame = Frame::with_content(frame_id, Content::Text(new_text.to_string()));
    tag.add_frame(frame);
    if let Err(err) = tag.write_to_path(path, Version::Id3v23) {
        return Err(Error::Id3Error(err));
    }
    Ok(())
}

pub fn read_tag(path: impl AsRef<Path>) -> Result<Tag> {
    match Tag::read_from_path(path) {
        Ok(tag) => Ok(tag),
        Err(id3::Error {
            kind: id3::ErrorKind::NoTag,
            ..
        }) => Ok(Tag::new()),
        Err(err) => {
            Err(Error::Id3Error(err))
        }
    }
}

pub fn run_ffmpeg<'a>(ffmpeg_path: &str, arguments: impl IntoIterator<Item = &'a str>) -> Result<()> {
    let status = match Command::new(ffmpeg_path).args(arguments).status() {
        Ok(status) => status,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(Error::FfmpegNotFoundError(ffmpeg_path.to_string()))
        }
        Err(err) => return Err(Error::IoError(err)),
    };
    match status.code() {
        Some(0) => {
            println!("Finished");
            Ok(())
        }
        Some(code) => Err(Error::FfmpegError(code)),
        None => Err(Error::FfmpegError(1)),
    }
}
