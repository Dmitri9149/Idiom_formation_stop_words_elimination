// handling text on the string stage
//
//use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
// use std::ops::Deref;
//use fancy_regex::Regex;
use crate::TextStage;

pub str Sentences {
    pub sentences: Vec<String>
}

// split TextStage string (text) on sentences using the crate
// UnicodeSegmentation and construct the Vector of Words where every
// sentence is treated as a big unique 'word'
//
    pub fn vocab_of_sentences(stage: &TextStage) -> Sentences{
        let sentences = stage
            .text
            .unicode_sentences()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        WordsVector {words:sentences}

    }





