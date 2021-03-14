// main functionality of the tokenizer
use entropy_tokenizer::text_stage::{TextStageOriginal, TextStage};
use entropy_tokenizer::sentence_stage::Sentences;
use entropy_tokenizer::sentence_stage_a::SentencesA;




fn main() {
// get text from a file 
    let txt_orig = TextStageOriginal::build_text_stage("data/Ulysses.txt");
    let mut txt = TextStage::from_original(&txt_orig);
    txt = txt.replace_string_to_string("\r\n"," ");
    let mut sentences = SentencesA::vocab_of_sentences_a(&txt);
    sentences.trim_sentences_a('A');

//    println!("The text : {:?}\n", txt.text);
    println!("The sentences : {:?}", &sentences.sentences[0..100]);
}

