// words to numbers and back 
use std::collections::btree_map::BTreeMap;
use crate::words_vocab::{WordsVocabAsBTree};
pub struct IndexToWords<'a> {
    pub index:BTreeMap<i32,&'a str>
}

impl IndexToWords <'_>{
    pub fn from_words_vocab_btree(voc:&BTreeMap, special_tokens:&mut Vec<&str>) -> IndexToWords {
        let mut temp_vector = voc.to_value_ordered_vector;
        temp_vector = special_tokens.append(temp_vector);
        
        IndexToWords {index:BTreeMap::new}

    }

}
