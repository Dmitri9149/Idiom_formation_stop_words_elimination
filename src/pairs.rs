// vocab of pairs, where pairs are pairs of words 
// and where words are represented as u32 indexes
use std::collections::HashMap;
//use crate::pair::{Pair};
use crate::sentence_stage::{IndicesCollection, VectorOfIndicesCollection};

pub struct Pairs {
    pub pairs:HashMap<(Vec<u32>,Vec<u32>),u32>
}

impl Pairs {
    pub fn new() -> Pairs {
        Pairs {pairs:HashMap::new()}
    }

    pub fn from_sentences_as_numbers(indices:&IndicesCollection) -> Pairs {
        let mut hsh = HashMap::new();
        for i in 0..indices.indices.len() { // indices.indices is Vec<Vec<u32>> 
            for j in 0..indices.indices[i].len()-1 { // indices.indices[i] is Vec<u32>
                let count = hsh
                    .entry((vec![indices.indices[i][j].to_owned()],vec![indices.indices[i][j+1].to_owned()]))
                    .or_insert(0);
                *count +=1;

            }
        }
        
        Pairs { pairs:hsh }
    }          

    pub fn from_sentences_as_wrapped_numbers(indices:&VectorOfIndicesCollection) -> Pairs {
        let mut hsh = HashMap::new();
        for i in 0..indices.indices.len() { // indices.indices is Vec<Vec<u32>> 
            for j in 0..indices.indices[i].len()-1 { // indices.indices[i] is Vec<u32>
                let count = hsh
                    .entry((indices.indices[i][j].to_owned(),indices.indices[i][j+1].to_owned()))
                    .or_insert(0);
                *count +=1;

            }
        }
        
        Pairs { pairs:hsh }
    }          


}


