use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;


//stage of text; text as a big string and the string processing
pub struct TextStage {
// original text
    pub text_0:str;
// text after some processing
    pub text_1:&str;
}

impl TextStage {
// build by reading a file, no a buffer
    pub fn build_text_stage(path: &str) -> TextStage {
        let mut f = File::open(path).unwrap();
        let &mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        TextStage {
            text_0:&contents,
            text1:&contents
        }
    }

}
