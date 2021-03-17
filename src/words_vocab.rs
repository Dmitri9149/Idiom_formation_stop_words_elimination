// the structure is for the words vocabulaty 
// collection of (word: quantity of the word ) pairs
use std::collections::HashMap;
use std::collections::btree_map::BTreeMap;
use crate::sentence_stage::{VectorOfWords};
use crate::words_to_numbers::{IndexToWords};


// as HashMap
pub struct WordsVocab {
    pub words:HashMap<String,u32>
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
    pub words:BTreeMap<String, u32>
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
    pub fn to_value_ordered_vector(&self) -> Vec<(String,u32)> {
        let mut vc:Vec<(String,u32)> = self
            .words
            .iter()
            .map(|x| (x.0.to_owned(), x.1.to_owned()))
            .collect();
        vc.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        vc
    }


}


// first number is index of a word (token) and second number is quantity 
// of the word in vocab
pub struct WordsAsNumbersVocab {
    pub words:Vec<(u32,u32)>
}


impl WordsAsNumbersVocab {
    pub fn from_words_vocab_btree(voc1:&WordsVocabAsBTree, voc2:&IndexToWords)-> WordsAsNumbersVocab{

// TODO -> check for the length equality and not empty of voc1 and voc2
        let mut words = Vec::new();
        for (key,value) in voc2.index
            .iter() {
            words.push((*key,*voc1.words.get(value).unwrap()));
        }
        WordsAsNumbersVocab {words:words}
    }
}

