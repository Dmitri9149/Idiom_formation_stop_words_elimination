// the structure is for the words vocabulaty 
// collection of (word: quantity of the word ) pairs
use std::collections::HashMap;
use std::collections::btree_map::BTreeMap;
use crate::sentence_stage::{VectorOfWords};


// as HashMap
pub struct WordsVocab {
    pub words:HashMap<String,i32>
}


impl WordsVocab {

    // build vocab from vector of words
    //
    pub fn vocab_from_vector(vec:&VectorOfWords) -> WordsVocab {
        let mut vocab= HashMap::new();
        for word in vec.words
            .iter() {
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
    pub fn vocab_from_vector(vec:&VectorOfWords) -> WordsVocabAsBTree {
        let mut vocab= BTreeMap::new();
        for word in vec.words.iter() {
            let count = vocab.entry(word.to_owned()).or_insert(0);
            *count +=1;           
        }

        WordsVocabAsBTree {words:vocab}
    }

// transform vocab to vector , order the vector in accordance of the 
// value (frequency)
    pub fn to_value_ordered_vector(&self) -> Vec<(String,i32)> {
        let mut vc:Vec<(String,i32)> = self
            .words
            .iter()
            .map(|x| (x.0.to_owned(), x.1.to_owned()))
            .collect();
        vc.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        vc
    }


}
