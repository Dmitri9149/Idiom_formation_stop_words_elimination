// handling text on the strings stage
// text is splitted on substrings in some way 
//
//
//use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
// use std::ops::Deref;
//use fancy_regex::Regex;
//use std::ops::Range;
use crate::text_stage::TextStage;
use crate::string_processing::separate_punctuation;
use crate::words_to_numbers::{IndexToWordsAsBTree};


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

// eliminate all empty strings from the colletion 
// it means no any empty subvectors in Vec of Vectors +
// eliminate all empty words from every sub Vector 
// it means no any empty words in sub Vectors of a total Vector
    pub fn no_empty_strings_and_words(&mut self) {
        self.collections = self.collections
            .iter()
            .map(|x| eliminate_empty_words(x.to_vec()))
            .collect::<Vec<Vec<String>>>();
        
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

// same as WordsCollection but words now are represented by their 
// indices
pub struct IndicesCollection {
    pub indices:Vec<Vec<u32>>
}

// take a WordsCollection and a WordsToNumbers structure and 
// produce Vector of Vector of indices 

impl IndicesCollection {
    pub fn from_words_collection(collection:&WordsCollection, words_to_index:&IndexToWordsAsBTree) 
        -> IndicesCollection {
            let res = collection.collections
                .iter()
                .map(|x| transform_collection_to_indices(x,&words_to_index))
                .collect();

            IndicesCollection {indices:res}

        }
}

pub struct VectorOfIndicesCollection {
    pub indices:Vec<Vec<Vec<u32>>>
}

impl VectorOfIndicesCollection {
    pub fn new() -> VectorOfIndicesCollection {
        VectorOfIndicesCollection {indices:Vec::new()}
    }

    pub fn from_indices_collection(indices:&IndicesCollection) 
        -> VectorOfIndicesCollection{
            let res = indices.indices
                .iter()
                .map(|x| crate::vec_to_vec_of_vec(x.to_vec()))
                .collect::<Vec<_>>();
            VectorOfIndicesCollection {indices:res}
            
    }

    pub fn transform_using_a_pair(&mut self, pair:&(Vec<u32>,Vec<u32>)) {
        self.indices = self.indices
            .iter()
            .map(|x| merge_pair_in_vec_of_vec(x.to_vec(),pair))
            .collect::<Vec<Vec<Vec<u32>>>>();
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

// some helper functions
// eliminate all empty Strings from Vector of Strings
//
pub fn eliminate_empty_words(vec:Vec<String>)-> Vec<String>{
    let res = vec
        .iter()
        .map(|x| x.to_owned())
        .filter(|x| x.is_empty() == false)
        .collect::<Vec<String>>();
    res.to_vec()
}
// take Vector of Words and WordsToIndex structure and transform words in the Vector 
// to the indices: Vector of words -> Vector of indices

pub fn transform_collection_to_indices(collection:&Vec<String>, indices:&IndexToWordsAsBTree) 
    -> Vec<u32>{
        let res = collection
            .iter()
            .map(|x| indices.word.get(x).unwrap())
            .map(|x| x.to_owned())
            .collect::<Vec<u32>>();

        res
}

pub fn merge_pair_in_vec_of_vec(mut vec_of_vec:Vec<Vec<u32>>, pair:&(Vec<u32>,Vec<u32>))
    -> Vec<Vec<u32>> {
    let mut ranges:Vec<_> = Vec::new();
    let size = &vec_of_vec.len();

    let mut begin = 0;
    let mut i = 0;
    while i < size-1 {
        if pair == &(vec_of_vec[i].to_owned(), vec_of_vec[i+1].to_owned()) {
            ranges.push(begin..i);
            begin = i+2;
            i = i+2;
        } else {
            i+=1;
        }
    }
/*
    let mut begin = 0;
    for mut i in 0..size-1 {
        if pair == &(vec_of_vec[i].to_owned(), vec_of_vec[i+1].to_owned()) {
            ranges.push(begin..i);
            begin = i+2;
            i = i+2;
        }
    }
*/  
    if begin !=0 {
        let mut res:Vec<Vec<_>> = Vec::new();
        let mut x:Vec<_>;
        for r in ranges {
            res.append(&mut vec_of_vec[r.start..r.end].to_vec());
            x = vec_of_vec[r.end].to_owned();
            x.append(&mut vec_of_vec[r.end+1]);
            res.push(x.to_owned());    
        } 
        res 
    } else {
        vec_of_vec.to_owned()
    }
}
