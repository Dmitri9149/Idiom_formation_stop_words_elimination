// main functionality of the tokenizer
use entropy_tokenizer::text_stage::{TextStageOriginal, TextStage};
use entropy_tokenizer::sentence_stage::{Sentences, WordsCollection};
use entropy_tokenizer::string_processing::{to_collection_split_on_space};
use entropy_tokenizer::words_vocab::{WordsVocab, WordsVocabAsBTree, WordsAsNumbersVocab};
use entropy_tokenizer::words_to_numbers::{IndexToWords};

//use entropy_tokenizer::sentence_stage_a::SentencesA;




fn main() {
// get text from a file 
    let txt_orig = TextStageOriginal::build_text_stage("data/Ulysses.txt");
    let mut txt = TextStage::from_original(&txt_orig);
//    txt = txt
//      .replace_string_to_string("\r\n\u{000C}\u{00D}\u{0085}\u{200E}\u{200F}\u{2028}\u{2029}"," ");
//    txt = txt
//      .replace_chars_to_char("\r\n\u{000C}\u{00D}\u{0085}\u{200E}\u{200F}\u{2028}\u{2029}",' ');
// horizontal tab
    txt = txt.replace_string_to_string("\u{0009}"," ");
// vertical tab
    txt = txt.replace_string_to_string("\u{0008}"," ");
// carriage return + line feed
    txt = txt.replace_string_to_string("\r\n"," ");
// form feed
    txt = txt.replace_string_to_string("\u{000C}"," ");
// carriage return
    txt = txt.replace_string_to_string("\u{000D}"," ");
// line feed
    txt = txt.replace_string_to_string("\u{000A}"," ");
// next line
    txt = txt.replace_string_to_string("\u{0085}"," ");
// left-to-right-mark
    txt = txt.replace_string_to_string("\u{200E}"," ");
// right-to-left-mark
    txt = txt.replace_string_to_string("\u{200F}"," ");
// line separator
    txt = txt.replace_string_to_string("\u{2028}"," ");
// paragraph separator
    txt = txt.replace_string_to_string("\u{2029}"," ");

//    txt = txt.remove_white_space();
    let mut sentences = Sentences::vocab_of_sentences(&txt);

//    println!("The text sentences: {:?}\n", &sentences.sentences[0..200]);

    sentences.trim_sentences(' ');
    sentences.separate_punctuation("?.!\"—,_:()[]");
    sentences.split_on_string(",");
    sentences.split_on_string(":");
    sentences.split_on_string(".");
    sentences.split_on_string("?");
    sentences.split_on_string("—");
    sentences.split_on_string("!");
    sentences.split_on_string("_");
    sentences.split_on_string(";");
    sentences.split_on_string("(");
    sentences.split_on_string(")");
    sentences.split_on_string("[");
    sentences.split_on_string("]");
    sentences.split_on_string("\"");
    sentences.split_on_string("[");
    sentences.split_on_string("]");

    sentences.trim_sentences(' ');
    sentences.no_empty_strings();

    let collection = WordsCollection::from_sentences(&sentences, to_collection_split_on_space);
//    let flatten_collection = WordsCollection::flatten_words_collections(&collection);
    let mut words_vector = WordsCollection::flatten_words_collections(&collection);
    words_vector.no_empty_strings();
//    let words_vocab = WordsVocab::vocab_from_vector(&words_vector);
    let words_vocab = WordsVocabAsBTree::vocab_from_vector(&words_vector);
    let ordered_vector = words_vocab.to_value_ordered_vector();

    let _special = vec!["eos".to_string(), "bos".to_string(), "none".to_string()];
    let empty = Vec::new();

    let index_vocab = IndexToWords::from_words_vocab(&words_vocab, empty);

    let word_as_number_vocab = WordsAsNumbersVocab::from_words_vocab_btree(&words_vocab, &index_vocab);

//    println!("The text : {:?}\n", txt_orig.text);
    println!("The sentences :\n{:?}\n", &sentences.sentences[0..200]);
    println!("The collections:\n{:?}\n", &collection.collections[0..200]);
//    println!("The words vector:\n{:?}\n", &words_vector);
//    println!("The words_vocab:\n{:?}\n", &words_vocab.words);
    println!("The number of words :{}", &words_vocab.words.keys().len());
    println!("The ordered (by frequency) vocab of words:\n{:?}\n", &ordered_vector[0..200]);
    println!("The index to words is :\n{:?}", &index_vocab.index[0..200]);
    println!("The words to index is :\n{:?}", &index_vocab.word[0..200]);
    println!("The vocab of index_of_word -> quantity of the word in vocab : \n{:?}"
             , &word_as_number_vocab.words[0..200]);

}

