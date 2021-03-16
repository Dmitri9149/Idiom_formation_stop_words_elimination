// the structure is for the words vocabulaty 
// collection of (word: quantity of the word ) pairs
use std::collections::HashMap;
use std::collections::btree_map::BTreeMap;
//use crate::sentence_stage::{WordsCollection};


// as HashMap
pub struct WordsVocab {
    pub words:HashMap<String,i32>
}


impl WordsVocab {

    // build vocab from vector of words
    //
    pub fn vocab_from_vector(vec:&Vec<String>) -> WordsVocab {
        let mut vocab= HashMap::new();
        for word in vec.iter() {
            let count = vocab.entry(word.to_owned()).or_insert(0);
            *count +=1;           
        }

        WordsVocab {words:vocab}
    }   
}

// vocab as BTreeMap
pub struct WordsVocabAsBTree {
    pub words:BTreeMap<String, i32>
}

impl WordsVocabAsBTree {
    // build vocab from vector of words
    //
    pub fn vocab_from_vector(vec:&Vec<String>) -> WordsVocabAsBTree {
        let mut vocab= BTreeMap::new();
        for word in vec.iter() {
            let count = vocab.entry(word.to_owned()).or_insert(0);
            *count +=1;           
        }

        WordsVocabAsBTree {words:vocab}
    }   

}
