// main functionality of the tokenizer
use entropy_tokenizer::text_stage::{TextStageOriginal, TextStage};

fn main() {
// get text from a file 
    let txt = TextStageOriginal::build_text_stage("data/Ulysses.txt");
    let txt_nxt = TextStage::from_original(&txt);
    println!("The text : {:?}\n", txt_nxt.text);
}

