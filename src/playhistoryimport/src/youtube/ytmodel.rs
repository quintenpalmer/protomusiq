#[derive(Debug)]
pub struct Entry {
    pub title: String,
    pub artist: String,
    pub watched: String,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct EntryKey {
    pub artist: String,
    pub title: String,
}

pub enum PromptResult<T> {
    Answer(T),
    NothingFound,
    Stop,
}
