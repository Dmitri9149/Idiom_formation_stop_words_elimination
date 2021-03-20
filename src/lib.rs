pub mod text_stage;
pub mod sentence_stage;
//pub mod sentence_stage_a;
pub mod string_processing;
pub mod words_vocab;
pub mod words_to_numbers;
pub mod pairs;
pub use crate::pairs as pairs_p;
pub use crate::text_stage as text;
pub use crate::sentence_stage as sentence;
//pub use crate::sentence_stage_a as sentence_a;
pub use crate::string_processing as str_proc;
use std::collections::HashMap;


// some helper functions
// take Vector of numbers and transform in Vector of Vector of numbers
fn vec_to_vec_of_vec(vec:Vec<u32>) -> Vec<Vec<u32>> {
    let size = vec.len();
    let mut vvv = Vec::with_capacity(size);
    let mut vv;
    for v in &vec {
        vv = vec![v.to_owned()];
        vvv.push(vv);
    }

    vvv
}

// the function return the key with biggest value
fn max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<(&K,&V)>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
//        .map(|(k, v)| (k,v))
}



/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_punctuation() {
        let sentence: &mut str = "some! text to U.S.A to test.";
        let result = "some !  text to U . S . A to test . ";
        assert_eq!(result, str_proc::separate_punctuation(sentence,"!.?"));

    }
}

*/
