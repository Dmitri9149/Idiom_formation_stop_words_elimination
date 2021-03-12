// main functionality of the tokenizer
use entropy_tokenizer::text_stage::TextStageOriginal;

fn main() {
// get text from a file 
    let txt = TextStageOriginal::build_text_stage("data/Ulysses.txt");
    println!("The text : {:?}\n", txt.text);
}

