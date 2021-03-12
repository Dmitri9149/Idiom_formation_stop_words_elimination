// main functionality of the tokenizer
use entropy_tokenizer::text_stage::TextStage;

fn main() {
// get text from a file 
    let txt = TextStage::build_text_stage("data/Ulysses.txt");
    println!("The text : {:?}\n", txt.text_0);
}

