// main functionality of the tokenizer
use entropy_tokenizer::text_stage::{TextStageOriginal, TextStage};
use entropy_tokenizer::sentence_stage::{Sentences, WordsCollection};
use entropy_tokenizer::string_processing::{to_collection_split_on_space};
//use entropy_tokenizer::sentence_stage_a::SentencesA;




fn main() {
// get text from a file 
    let txt_orig = TextStageOriginal::build_text_stage("data/Ulysses.txt");
    let mut txt = TextStage::from_original(&txt_orig);
    txt = txt.replace_string_to_string("\r\n"," ");
    let mut sentences = Sentences::vocab_of_sentences(&txt);

//    println!("The text sentences: {:?}\n", &sentences.sentences[0..200]);

    sentences.trim_sentences(' ');
    sentences.separate_punctuation("?.!\"—,_:");
    sentences.split_on_string(",");
    sentences.split_on_string(":");
    sentences.split_on_string(".");
    sentences.split_on_string("?");
    sentences.split_on_string("—");
    sentences.split_on_string("!");
    sentences.split_on_string("_");
    sentences.split_on_string(";");
    sentences.trim_sentences(' ');
    sentences.no_empty_strings();

    let collection = WordsCollection::from_sentences(&sentences, to_collection_split_on_space);


    



//    println!("The text : {:?}\n", txt.text);
    println!("The sentences :\n{:?}\n", &sentences.sentences[0..200]);
    println!("The collections:\n{:?}\n", &collection.collections[0..200]);
}

