use std::io::prelude::*;
use std::fs::File;
//use std::collections::HashMap;


//stage of text; text as a big string
// mostly for to keep the original text
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

 // change a chars from a list (the list is represented as 
 // a string) to another char
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


// insert ' ' between punctuation marks '!,.' and a word 
//
    pub fn separate_punctuation(self, s:&str) -> TextStage {
        let mut new_str = String::new();

        let mut it = self.text.chars().peekable();

        while let Some(current) = it.next() {
            if let Some(&next) = it.peek() {
                if current != ' ' &&  s.contains(next) {
                    new_str.push(current);
                    new_str.push(' ');
                }  else { new_str.push(current) }
            }
        }
        TextStage {text: new_str, ..self}
    }
// to lowercase all the string
    pub fn to_lowercase(self) -> TextStage {
        let text = self.text.to_lowercase();
        TextStage { text: text, ..self }
    }

// replace new line symbols by space (\t \n \r)
    pub fn replace_new_line(self) -> TextStage {
        let mut text = self.text.replace('\t',&' '.to_string()); // '\t'
//        self.text1.replace('\t',&' '.to_string()); // '\t'
        text = text.replace('\n',&' '.to_string()); // '\n'
        text = text.replace('\r', &' '.to_string());


        TextStage {text:text,  ..self }
    }

    pub fn remove_white_space(self) -> TextStage {
        let text = self.text
            .chars()
            .map(|x| -> char {
                if x.is_whitespace() {
                    ' '
                } else { x }
            })
            .collect();

        TextStage {text:text, ..self}
    }


//replace all occurrences of one string within another 
//string 
   pub fn replace_string_to_string(self, sub1:&str,sub2:&str) -> TextStage{
        let result = self.text.replace(sub1,sub2);
        TextStage {text:result}
    }


}
