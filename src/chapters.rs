use crate::{read_tag, run_ffmpeg, Error, Result};
use core::str;
use id3::TagLike;
use prettytable::{row, Table};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fmt::Display,
    io::{self, Write},
    ops::{Index, IndexMut},
    path::{Path, PathBuf},
    process::Command,
    slice::{Iter, IterMut},
};
use tempfile::NamedTempFile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    title: String,
    start: u32,
    end: u32,
}

impl Chapter {
    pub fn new(title: impl Into<String>, start: u32, end: u32) -> Self {
        Self {
            title: title.into(),
            start,
            end,
        }
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_start(&mut self, start: u32) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: u32) {
        self.end = end;
    }

    pub fn ffmetadata(&self) -> String {
        format!(
            "\n\
            [CHAPTER]\n\
            TIMEBASE=1/1000\n\
            START={}\n\
            END={}\n\
            title={}\n\
",
            self.start, self.end, self.title
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterList {
    title: String,
    author: String,
    chapters: Vec<Chapter>,
}

impl ChapterList {
    pub fn new(title: impl Into<String>, author: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            author: author.into(),
            chapters: Vec::new(),
        }
    }

    pub fn from_path_set(
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
        title: impl Into<String>,
        author: impl Into<String>,
    ) -> Result<Self> {
        let mut chapters: Vec<Chapter> = Vec::new();
        let mut playhead: u32 = 0;

        for (i, path) in paths.into_iter().enumerate() {
            let tag = read_tag(&path)?;
            let chapter_title = tag.title().unwrap_or(&i.to_string()).to_string();
            let duration = mp3_duration::from_path(path)?.as_millis() as u32;
            let start = playhead;
            let end = playhead + duration;

            chapters.push(Chapter::new(chapter_title, start, end));

            playhead = end;
        }
        Ok(Self {
            title: title.into(),
            author: author.into(),
            chapters,
        })
    }

    pub fn from_chaptered_file(path: &str) -> Result<ChapterList> {
        let path = path.as_ref();
        let arguments = [
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_chapters",
            path,
        ];
        let output = match Command::new("ffprobe").args(arguments).output() {
            Ok(output) => output,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return Err(Error::FfprobeNotFoundError());
            }
            Err(err) => return Err(Error::IoError(err)),
        };
        let tag = read_tag(&PathBuf::from(path))?;
        let title = tag.title().unwrap_or("Unknown title").to_string();
        let author = tag.artist().unwrap_or("Unknown author").to_string();
        let mut chapter_list = ChapterList::new(title, author);

        let chapters_json: Value = serde_json::from_str(
            str::from_utf8(&output.stdout).map_err(|_| Error::ChapterReadError)?,
        )
        .unwrap();
        for json_chapter in chapters_json["chapters"]
            .as_array()
            .ok_or(Error::ChapterReadError)?
        {
            let title: &str = json_chapter["tags"]["title"]
                .as_str()
                .ok_or(Error::ChapterReadError)?;
            let start: u32 = json_chapter["start"]
                .as_i64()
                .ok_or(Error::ChapterReadError)? as u32;
            let end: u32 = json_chapter["end"]
                .as_i64()
                .ok_or(Error::ChapterReadError)? as u32;

            chapter_list.push(Chapter::new(title, start, end));
        }
        Ok(chapter_list)
    }

    pub fn ffmetadata(&self) -> String {
        let mut ffmetadata = format!(
            ";FFMETADATA\n\
            title={}\n\
            artist={}\n\
            genre=AudioBook\n\
",
            self.title, self.author
        );
        for chapter in &self.chapters {
            ffmetadata.push_str(&chapter.ffmetadata());
        }
        ffmetadata
    }

    pub fn from_toml(toml: &str) -> Result<Self> {
        let chapter_list: ChapterList = toml::from_str(toml)?;
        Ok(chapter_list)
    }

    pub fn write_to_file(
        &self,
        input_path: &str,
        output_path: &str,
        ffmpeg_path: &str,
    ) -> Result<()> {
        let ffmetadata: String = self.ffmetadata();

        let mut ffmetadata_tmp = NamedTempFile::new()?;
        ffmetadata_tmp.write_all(ffmetadata.as_bytes())?;
        let ffmetadata_tmp_path = ffmetadata_tmp.path().to_string_lossy();

        let arguments = [
            "-i",
            input_path,
            "-i",
            &ffmetadata_tmp_path,
            // "-map",
            // "0",
            "-map_metadata",
            "1",
            "-map_chapters",
            "1",
            "-c",
            "copy",
            output_path,
        ];
        run_ffmpeg(ffmpeg_path, arguments)?;

        Ok(())
    }

    pub fn toml(&self) -> Result<String> {
        toml::to_string(self).map_err(|err| Error::TomlSerializationError(err))
    }

    pub fn iter(&self) -> Iter<'_, Chapter> {
        self.chapters.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Chapter> {
        self.chapters.iter_mut()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn author(&self) -> String {
        self.author.clone()
    }

    pub fn len(&self) -> usize {
        self.chapters.len()
    }

    pub fn push(&mut self, new_chapter: Chapter) {
        self.chapters.push(new_chapter);
    }

    pub fn insert(&mut self, index: usize, new_chapter: Chapter) {
        self.chapters.insert(index, new_chapter);
    }

    pub fn remove(&mut self, index: usize) -> Chapter {
        self.chapters.remove(index)
    }
}

impl Index<usize> for ChapterList {
    type Output = Chapter;

    fn index(&self, index: usize) -> &Chapter {
        &self.chapters[index]
    }
}

impl IndexMut<usize> for ChapterList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.chapters[index]
    }
}

impl Display for ChapterList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.set_titles(row![
            b->"Title",
            b->"Start (ms)",
            b->"End (ms)",
        ]);
        for chapter in &self.chapters {
            table.add_row(row![chapter.title(), chapter.start(), chapter.end()]);
        }
        write!(f, "{}", table.to_string())
    }
}
