// text is splitted on sentences and saved in Vector
// + some operations for the sentences processing
// here we use &str to save sentences , not String
//use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
// use std::ops::Deref;
//use fancy_regex::Regex;
use crate::text_stage::TextStage;
use crate::string_processing::{separate_punctuation};

pub struct SentencesA<'a> {
    pub sentences: Vec<&'a str>
}

// split TextStage string (text) on sentences using the crate
// UnicodeSegmentation and construct the Vector of sentences where every
// sentence is treated as a big unique 'word'
//
impl SentencesA<'_> { 
    pub fn vocab_of_sentences_a(stage: &TextStage) -> SentencesA{
        let sentences = stage
            .text
            .unicode_sentences()
//            .map(|x| x.to_owned())
            .collect::<Vec<&str>>();
        SentencesA {sentences:sentences}

    }

// trim the sentences in the vector from some characters

    pub fn trim_sentences_a(&mut self, ch:char) {
        self.sentences = self.sentences
            .iter()
            .map(|x| x.trim_matches(ch))
//            .to_owned())
            .collect::<Vec<&str>>();
    }

    pub fn separate_punctuation_a(&mut self, st:&str) {
        self.sentences = self.sentences
            .iter()
            .map(|x| x.replace(st, " "))
            .collect::<Vec<&str>>();
    }

}




