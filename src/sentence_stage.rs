// handling text on the string stage
//
//use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
// use std::ops::Deref;
//use fancy_regex::Regex;
use crate::text_stage::TextStage;
use crate::string_processing::separate_punctuation;


pub struct Sentences {
    pub sentences: Vec<String>
}

// split TextStage string (text) on sentences using the crate
// UnicodeSegmentation and construct the Vector of Words where every
// sentence is treated as a big unique 'word'
//
impl Sentences { 
    pub fn vocab_of_sentences(stage: &TextStage) -> Sentences{
        let sentences = stage
            .text
            .unicode_sentences()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        Sentences {sentences:sentences}

    }

// trim the sentences in the vector from some characters

    pub fn trim_sentences(&mut self, ch:char) {
        self.sentences = self.sentences
            .iter()
            .map(|x| x.trim_matches(ch).to_owned())
            .collect::<Vec<String>>();
    }

    pub fn separate_punctuation(&mut self, st:&str) {
        self.sentences = self.sentences
            .iter()
            .map(|x| separate_punctuation(x.to_string(),st))
            .collect::<Vec<String>>();
    }


}




