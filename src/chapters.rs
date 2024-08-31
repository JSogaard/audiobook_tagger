use crate::{read_tag, Error};
use id3::TagLike;
use std::{
    ops::{Index, IndexMut},
    path::PathBuf, slice::{Iter, IterMut},
};

#[derive(Debug, Clone)]
pub struct Chapter {
    title: String,
    start: u32,
    end: u32,
}

impl Chapter {
    pub fn new(title: String, start: u32, end: u32) -> Self {
        Self { title, start, end }
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

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_start(&mut self, start: u32) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: u32) {
        self.end = end;
    }

    pub fn ffmetadata(&self) -> String {
        format!(
            "
[CHAPTER]
TIMEBASE=1/1000
START={}
END={}
title={}
",
            self.start, self.end, self.title
        )
    }
}

#[derive(Debug, Clone)]
pub struct ChapterList {
    title: String,
    author: String,
    chapters: Vec<Chapter>,
}

impl ChapterList {
    pub fn new(title: String, author: String) -> Self {
        Self {
            title,
            author,
            chapters: Vec::new(),
        }
    }

    pub fn from_path_set<'a>(
        paths: impl Iterator<Item = &'a PathBuf>,
        title: String,
        author: String,
    ) -> Result<Self, Error> {
        let mut chapters: Vec<Chapter> = Vec::new();
        let mut playhead: u32 = 0;

        for (i, path) in paths.enumerate() {
            let tag = read_tag(&path)?;
            let chapter_title = tag.title().unwrap_or(&i.to_string()).to_string();
            let duration = mp3_duration::from_path(path)?.as_millis() as u32;
            let start = playhead;
            let end = playhead + duration;

            chapters.push(Chapter {
                title: chapter_title,
                start,
                end,
            });

            playhead = end;
        }
        Ok(Self {
            title,
            author,
            chapters,
        })
    }

    pub fn ffmetadata(&self) -> String {
        let mut ffmetadata = format!(
            ";FFMETADATA
title={}
artist={}
genre=AudioBook
",
            self.title, self.author
        );
        for chapter in &self.chapters {
            ffmetadata.push_str(&chapter.ffmetadata());
        }
        ffmetadata
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
