pub mod word_enum;
use crate::word_enum::*;

pub struct Vocabulary {
    word: String,
}

impl Vocabulary {
    pub fn new(word: String) -> Self {
        Vocabulary { word }
    }

    pub fn parse(&self) -> Option<WordEnum> {
        self.word
    }
}
