pub mod text_stage;
pub mod sentence_stage;
//pub mod sentence_stage_a;
pub mod string_processing;
pub use crate::text_stage as text;
pub use crate::sentence_stage as sentence;
//pub use crate::sentence_stage_a as sentence_a;
pub use crate::string_processing as str_proc;

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
