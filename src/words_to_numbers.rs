// words to numbers and back 
use std::collections::btree_map::BTreeMap;
use crate::words_vocab::{WordsVocabAsBTree};
// use std::collections::btree_map::Entry::{Occupied, Vacant};
pub struct IndexToWords {
    pub index:BTreeMap<usize,String>,
    pub word:BTreeMap<String,usize>
}

impl IndexToWords {
    pub fn from_words_vocab(voc:&WordsVocabAsBTree, mut special_tokens:Vec<String>) 
        -> IndexToWords {
        if voc.words.keys().len() == 0 {
            panic!("Empty words vocabulary ! Panic, crash program!");
        }
        let temp_vector = &mut voc.to_value_ordered_vector()
            .iter()
            .map(|x| &(x.0)[..])
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        special_tokens.append(temp_vector);
        let mut index = BTreeMap::new();
        let mut word = BTreeMap::new();
        let mut wd;
        for i in 0..special_tokens.len() {
            wd = &special_tokens[i];
            index.insert(i,wd.to_string());
            word.insert(wd.to_string(),i);
        }
        
        IndexToWords {index:index, word:word}

    }

}


