// words to numbers and back 
use std::collections::btree_map::BTreeMap;
use crate::words_vocab::{WordsVocabAsBTree};
pub struct IndexToWords {
    pub index:BTreeMap<i32,String>
}

impl IndexToWords {
    pub fn from_words_vocab_btree(voc:&WordsVocabAsBTree, mut special_tokens:Vec<String>) 
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
        
        IndexToWords {index:BTreeMap::new()}

    }

}


