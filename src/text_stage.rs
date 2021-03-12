use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;


//stage of text; text as a big string and the string processing
pub struct TextStageOriginal {
// original text
    pub text:String
}


impl TextStageOriginal {
// build by reading a file, no a buffer
    pub fn build_text_stage(path: &str) -> TextStageOriginal {
        let mut f = File::open(path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        TextStageOriginal {
            text:contents.clone()
        }
    }

//    pub fn from_initial(self) -> TextStage {
//        TextStage {
//            text_0:self.text_0,
//            text_1:&self.text_0,
//
//        }
//    }

}
