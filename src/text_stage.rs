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
}

pub struct TextStage {
    pub text:String
}
// build TextStage (we can process strings here) from 
// TextStageOriginal where we keep the original text-string
impl TextStage {
    pub fn from_original(orig:&TextStageOriginal) -> TextStage {
        TextStage {
            text:orig.text.to_owned()
        }
        
    }

// change a char to another char
    pub fn replace_char_to_char(self, x:char, y:char) -> TextStage {
        let xx = x;
        let yy = y;
        let new_text = self.text.chars()
            .map(|x| -> char {
                if x==xx {
                    yy
                } else {x}
            })
        .collect();

        TextStage {
            text:new_text, ..self
        }
    }

 // change a chars from a list to another char
    pub fn replace_chars_to_char(self, aa:&str, b:char) -> TextStage {
        let text = self.text.chars()
            .map(|x| -> char {
                if aa.contains(x) {
                    b
                } else {x}
            })
        .collect();

        TextStage {
            text:text, ..self
        }
    }


}
