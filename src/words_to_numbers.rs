// words to numbers and back 
//use std::collections::btree_map::BTreeMap;
use crate::words_vocab::{WordsVocabAsBTree};
// use std::collections::btree_map::Entry::{Occupied, Vacant};
pub struct IndexToWords {
    pub index:Vec<(u32,String)>,
    pub word:Vec<(String,u32)>
}

impl IndexToWords {
    pub fn from_words_vocab(voc:&WordsVocabAsBTree, mut special_tokens:Vec<String>) -> IndexToWords{
        if voc.words.keys().len() == 0 {
            panic!("Empty words vocabulary ! Panic, crash program!");
        }
        let temp_vector = &mut voc.to_value_ordered_vector()
            .iter()
            .map(|x| &(x.0)[..])
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        special_tokens.append(temp_vector);
        let mut index = Vec::new();
        let mut word = Vec::new();
        let mut i = 0;
        for tok in special_tokens.iter() {
            index.push((i,tok.to_string()));
            word.push((tok.to_string(),i));
            i = i+1;
        }  
        IndexToWords {index:index, word:word}
    }
}


