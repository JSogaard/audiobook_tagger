use crate::{Error, Result};
use clap::parser::ValuesRef;
use id3::{Content, Frame, Tag, TagLike, Version};
use std::{collections::BTreeSet, path::{Path, PathBuf}};

// pub fn generate_metadata(
//     paths: &BTreeSet<PathBuf>,
//     title: &str,
//     author: &str,
// ) -> Result<String> {
//     let mut ffmetadata: String = format!(
//         ";FFMETADATA
// title={title}
// artist={author}
// genre=AudioBook
// "
//     );
//     let mut playhead: u32 = 0;

//     for path in paths {
//         let tag = read_tag(path)?;
//         let chapter_title = tag.title().unwrap_or("Chapter");
//         let duration = mp3_duration::from_path(path)?.as_millis() as u32;
//         let start = playhead;
//         let end = playhead + duration;

//         ffmetadata.push_str(&format!(
//             "
// [CHAPTER]
// TIMEBASE=1/1000
// START={start}
// END={end}
// title={chapter_title}
// "
//         ));
//         playhead = end;
//     }
//     Ok(ffmetadata)
// }

pub fn expand_wildcards(raw_paths: ValuesRef<String>) -> Result<BTreeSet<PathBuf>> {
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
    let mut tag: Tag = read_tag(&path)?;
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
            return Err(Error::Id3Error(err));
        }
    }
}
