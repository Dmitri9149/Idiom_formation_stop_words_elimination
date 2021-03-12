// main functionality of the tokenizer
use bpe::text_stage::TextStage

fn main {
// get text from a file 
    let txt = TextStage::build_text_stage("data/ullyses.txt");
    println!("The text : {}\n", txt.text_1);
}

