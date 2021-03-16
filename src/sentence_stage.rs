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
// separate punctuation
    pub fn separate_punctuation(&mut self, st:&str) {
        self.sentences = self.sentences
            .iter()
            .map(|x| separate_punctuation(x.to_string(),st))
            .collect::<Vec<String>>();
    }
// split sentencs at some 'string' (like split at "," comma)
    pub fn split_on_string(&mut self, st:&str) {
        self.sentences = self.sentences
            .iter()
            .flat_map(|x| x.split(st))
            .map(str::to_owned)
            .collect::<Vec<_>>();
    }
// eliminate all empty strings from collection
    pub fn no_empty_strings(&mut self) {
        self.sentences = self.sentences
            .iter()
            .map(|x| x.to_string())
            .filter (|x| x.is_empty()== false)
            .collect::<Vec<String>>();
    }

}
// split sentences to words and keep as vector of words collections
pub struct WordsCollection {
    pub collections:Vec<Vec<String>>
}

impl WordsCollection {
    pub fn from_sentences(sn:&Sentences, f:fn(String)-> Vec<String>) -> WordsCollection{
        let res = sn.sentences
            .iter()
            .map(|x| f(x.to_string()))
            .collect::<Vec<Vec<String>>>();
        WordsCollection {collections:res}
    }
// flatten words collections to vector of words
    pub fn flatten_words_collections(cn:&WordsCollection)-> VectorOfWords {
        let res = cn.collections
            .iter()
            .flatten()
            .map(|x| x.to_owned())
            .collect();

        VectorOfWords {words:res}
    }
}

// vector of unordered, may be repeated words
pub struct VectorOfWords {
    pub words:Vec<String>
}

impl VectorOfWords {
    pub fn no_empty_strings(&mut self) {
        self.words = self.words
            .iter()
            .map(|x| x.to_owned())
            .filter(|x| x.is_empty() == false)
            .collect::<Vec<String>>();

    }
}


