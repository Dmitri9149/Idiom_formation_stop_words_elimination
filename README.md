In the work tokenizer similar in structure to Byte Pair Encoding tokenizer (see  
https://arxiv.org/pdf/1508.07909.pdf original article for the BPE) is used for text analysis at the level of sentences. (The toknizer is written here in Rust).
Originally BPE was used for splitting of words.
The text is James Joyse "Ulysses"" from the Gutemberg Project
http://www.gutenberg.org/ebooks/4300 .

Tokenizers find the most frequent pairs of 'tokens' and generate a new token which may be represented as 'flattening' or 'merging' of the previous two tokens: ('i','ng') -> 'ing' , for example, if we run the tokenizer at the level of chars and words. If we run the tokenizer at the level of words, we will see how 'stop' words like 'the' are quickly paired with other words: ('of','the') -> 'of the'. The big resourse of 'the' is quickly splitted to 'of the' , 'in the', 'to the'. The similar behaviour is for 'a' and many other 'stop' words. 
Actually we may see the process of 'stop' words elimination. The words are not 'uselles' but their natural meaning arise is in combination with other words. The 'stop' words are resourses for 'idioms' and have to used as the consolidated words combinations. Outside such combinations, the words may be really meaningless. 
My experience is that we must be very explicit and clever with 'borders': the tokenization process is to be constrained by 'natural borders': border of words (if our tokens are chars and chars generated tokens) or by 'sentence' borders, if we run tokenizer at the level of words and words colections within 'sentences'. 

In the work the  text was split on sentences. But it is not enought: actual "borders" most probably correspond to punctuations. And sentences was additionally split on some punctuation marks (see the code for more details). 

At the stage we are getting collections of words like :.......["Will", "you", "come", "if", "I", "can", "get", "the", "aunt", "to", "fork", "out", "twenty", "quid"], ["He", "laid", "the", "brush", "aside", "and"], ["laughing", "with", "delight"], ["cried"], ["Will", "he", "come"], ["The", "jejune", "jesuit"], ["Ceasing"], ["he", "began", "to", "shave", "with", "care"], ["Tell", "me"], ["Mulligan"], ["Stephen", "said", "quietly"], ["Yes"], ["my", "love"], ["How", "long", "is", "Haines", "going", "to", "stay", "in", "this", "tower"], ["Buck", "Mulligan", "showed", "a", "shaven", "cheek", "over", "his", "right", "shoulder"], ["God"], ["isn’t", "he", "dreadful"], ["he", "said", "frankly"], ["A", "ponderous", "Saxon"], ["He", "thinks", "you’re", "not", "a", "gentleman"], ["God"], ["these", "bloody", "English"], ["Bursting", "with", "money", "and", "indigestion"], ["Because", "he", "comes", "from", "Oxford"], ["You", "know"], ["Dedalus"], ["you", "have", "the", "real", "Oxford", "manner"], ["He", "can’t", "make", "you", "out"], ["O"], ["my", "name", "for", "you", "is", "the", "best"], ["Kinch"], ["the", "knife-blade"], ["He", "shaved", "warily", "over", "his", "chin"], ["He", "was", "raving", "all", "night", "about", "a", "black", "panther"], ["Stephen", "said"], ["Where", "is", "his", "guncase"], ["A", "woful", "lunatic"], ["Mulligan", "said"], ["Were", "you", "in", "a", "funk"], ["I", "was"], ["Stephen", "said", "with", "energy", "and", "growing", "fear"], ["Out", "here", "in", "the", "dark", "with", "a", "man", "I", "don’t", "know", "raving", "and", "moaning", "to", "himself", "about", "shooting", "a", "black", "panther"], ["You", "saved", "men", "from", "drowning"], ["I’m", "not", "a", "hero"], ["however"], ["If", "he", "stays", "on", "here", "I", "am", "off"], ["Buck", "Mulligan", "frowned", "at", "the", "lather", "on", "his", "razorblade"], ["He", "hopped", "down", "from", "his", "perch", "and", "began", "to", "search", "his", "trouser", "pockets", "hastily"], ["Scutter"], ["he", "cried", "thickly"]..........

The : ["Buck", "Mulligan", "frowned", "at", "the", "lather", "on", "his", "razorblade"], ["He", "hopped", "down", "from", "his", "perch", "and", "began", "to", "search", "his", "trouser", "pockets", "hastily"], ["Scutter"], ["he", "cried", "thickly"] is example of the 'words collection'. 

Our tokenizer will generate a new tokens from original words collections. The first token which is generated is: 
(["of"], ["the"]) with quantity 1640 , the second is : (["in"], ["the"]) -> 1343 ; after about 70 iterations the 
first three words token is generated : (["says"], ["the", "citizen"]) with frequency about 50.  
As we can see it is merging of 'says' and previously generated 'the citizen'. 

We may think about the tokens as (partly) 'text specific natural idioms'. Some idioms like 'of the' for sure just 
reflect the English text internal structure. It is quite natural, because the "Ulysses" is written in English. But some, like ' says the citizen',  are for sure very text spacific. 

We may think also about the idioms as a new (partly) text specific 'vocab'. Such idioms are to be considered as 'indivisible' pairs in text analysis, and may characterize a text much better , than just a single words. 'of the ' and 
'of a' have very specific and quite clear meaning. But if we will take 'the',  'a' just a separate words we will 
quickly come to the 'stop' words notion and the general receips in text analysis: the words is better to eliminate from specific text analyses, there are many of such words, the words are spread within a text and adds nothing to a meaning.... etc.

But the tokenization process may change the status of the words. If you will check the output of the programm, 
you will see there are: ("the", 13676), ("of", 8171), ("and", 6695), ("a", 5885), ("to", 4873),  
("in", 4726), ("his", 3063), ("he", 2880), ("I", 2683), ("that", 2415), ("with", 2414), ("it", 2145), ("was", 2094), 
("on", 2025), ("for", 1836), ("her", 1686), ("you", 1674), ("him", 1507), ("is", 1372), ("at", 1232)... where the 
second digit is quantity of the word in the text. 

At the first iteration 'the' and 'of' converge and form the new token 'of the' with quantity 1640. But it is only a part of the resourse of 'the' and 'of'. The rest of 13676 pcs of 'the' will be very soon separated between 'in', 'to', 'on'. There will be  (["in"], ["the"]) (1343 pcs); (["on"], ["the"]) (606 pcs); (["to"], ["the"]) (561 pcs). 
And: 
1. The new tokens are much much more meaningfull.  
2. The tokens are much much more seldom. 

Where and how we may use the 'words tokens'. 

Here are some suggestions: 
1. In any text analysis the generation of the idioms may improove the quality of the analysis. 
2. In machine translation the idioms may/are to be translated 'as a whole' to similar idiom of target 
language. 'of the' has consolidated meaning and we may expect the presence of similar consolidated meaning - idiom 
in a target language. In the Transformer like scheme , when we generate translation of target sentence word by word , starting from the first word, may decrease the quantity of stages and increase quality of next word prediction. 
3. We may try to generate a 'natural basis' for example for machine translation and text analysis. 
Let us start from words and characters. We can split words to collection of characters, sentences to collection of 
characters too etc..... Every character will be basis vector in out system.   
Why we can not do this ? In the basis system the 'd o g' and 'g o d' words will be the same vector. 

But tokenization process at the character level will very soon generate a new representation for the words : 
something like 'do g' and go d', where 'do' and 'go' will be our new basis vectors in addition to original 'g' and 'd'. If we will iterate long enought, the representation of words in out new system of 'basic tokens' will be one to one: even the words will be still 'bag of tokens', because we can permute the vectors in the sum of vectors. 'do g' will still coinsides with 'g do' in 
vector representation.  But the 'g do' is not a word in English! There is very strong summetry breacking in any language, like 'or' is a word and 'ro' is not, and my hyposesis is the new tokens will make the symmetry breacking even stronge. Tokenization process gives a new pover to 'bag of words', where we think about 'bag' as sum of basic 'tokens' vectors. 
Even more. 'With Dmitri' in English is 'Dmitrikanssa' in Finnish, where we may see strong interplay of tokens as chars and tokens as words. We may proceeds as follows. 
a. Start from words as characters collections and generate a new tokens from characters. The original (chars) and newly generated tokens will be our new basis vectors. What is very important: some tokens will be actually our words: 
't' 'h' will combine in 'th' at the first iterations and very soon a new token 'the' will be actually generated. Quite many tokens will be actually out words. 
b. Start from sentences as words collections. Generate the new tokens - 'idioms' (what is done in the example work). Add the new idioms as additional orthogonal vectors to the basis vectors. SPLITT THE REST OF WORDS IN CHARACTERS...TOKENS BASIS. 
Our basis will looks like this: "a", "b"......"ing", ..., "the",... "token", "iz", "ation"..., "in the", ..., 
"of the", " get rid of"  ....
We can the basis for a text analysis and machine translation.
The big question is HOW MANY ITERATIONs (MERGES) WE HAVE TO MAKE. I do not discuss the topic here. Let us keep it as 'hyperparameter' which we have to find in some way. For the chars...words tokenizer,one  of the possibilities is as follows. At every iteration our vacabulary of tokens: (token, token quantity) is changing. We are getting a new tokens, some old tokens may disappear. We can calculate entropy of the vocabulary at every stage. The entropy is quickly run at the beginning and then reach a plato or even begin to decrease (has maximun at some iteration). The iteration where we reach a plato or maximum of the entropy is a one possible choice for the 'best number of iterations'. 
This is the topic of my current researches. 

It is possible to run examples with : 
`cargo run --example en_tokenizer` (en_tokenizer) is the name of example. 
But the work is mostly for experiments (I have to run a lot to make an algorythm), still not for a production of a ready system. 

Below is a sample output from the programm. The number of iteration is about 600. At about 550..600 iteration the quantity of newly generated tokens is about 10 only. The first iteration generate 'of the' token with 1640 pcs. 

```
~>/entropy_tokenizer$ cargo run --example en_tokenizer
   Compiling entropy_tokenizer v0.1.0 (/home/dmitri/entropy_tokenizer)
warning: variable does not need to be mutable
  --> examples/en_tokenizer.rs:89:9
   |
89 |     let mut collection_of_senteces_as_wrapped_indices =
   |         ----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: 1 warning emitted

    Finished dev [unoptimized + debuginfo] target(s) in 3.73s
     Running `target/debug/examples/en_tokenizer`
The first 200 sentences :
["\u{feff}The Project Gutenberg eBook of Ulysses", "by James Joyce  This eBook is for the use of anyone anywhere in the United States and most other parts of the world at no cost and with almost no restrictions whatsoever", "You may copy it", "give it away or re-use it under the terms of the Project Gutenberg License included with this eBook or online at www", "gutenberg", "org", "If you are not located in the United States", "you will have to check the laws of the country where you are located before using this eBook", "Title", "Ulysses  Author", "James Joyce  Release Date", "December 27", "2001", "eBook #4300", "Most recently updated", "December 27", "2019", "Language", "English  Character set encoding", "UTF-8  Produced by", "Col Choat and David Widger  *** START OF THE PROJECT GUTENBERG EBOOK ULYSSES ***", "Illustration", "Ulysses   by James Joyce   Contents", "I", "1", "2", "3", "II", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "III", "16", "17", "18", "I", "1", "Stately", "plump Buck Mulligan came from the stairhead", "bearing a bowl of lather on which a mirror and a razor lay crossed", "A yellow dressinggown", "ungirdled", "was sustained gently behind him on the mild morning air", "He held the bowl aloft and intoned", "Introibo ad altare Dei", "Halted", "he peered down the dark winding stairs and called out coarsely", "Come up", "Kinch", "Come up", "you fearful jesuit", "Solemnly he came forward and mounted the round gunrest", "He faced about and blessed gravely thrice the tower", "the surrounding land and the awaking mountains", "Then", "catching sight of Stephen Dedalus", "he bent towards him and made rapid crosses in the air", "gurgling in his throat and shaking his head", "Stephen Dedalus", "displeased and sleepy", "leaned his arms on the top of the staircase and looked coldly at the shaking gurgling face that blessed him", "equine in its length", "and at the light untonsured hair", "grained and hued like pale oak", "Buck Mulligan peeped an instant under the mirror and then covered the bowl smartly", "Back to barracks", "he said sternly", "He added in a preacher’s tone", "For this", "O dearly beloved", "is the genuine Christine", "body and soul and blood and ouns", "Slow music", "please", "Shut your eyes", "gents", "One moment", "A little trouble about those white corpuscles", "Silence", "all", "He peered sideways up and gave a long slow whistle of call", "then paused awhile in rapt attention", "his even white teeth glistening here and there with gold points", "Chrysostomos", "Two strong shrill whistles answered through the calm", "Thanks", "old chap", "he cried briskly", "That will do nicely", "Switch off the current", "will you", "He skipped off the gunrest and looked gravely at his watcher", "gathering about his legs the loose folds of his gown", "The plump shadowed face and sullen oval jowl recalled a prelate", "patron of arts in the middle ages", "A pleasant smile broke quietly over his lips", "The mockery of it", "he said gaily", "Your absurd name", "an ancient Greek", "He pointed his finger in friendly jest and went over to the parapet", "laughing to himself", "Stephen Dedalus stepped up", "followed him wearily halfway and sat down on the edge of the gunrest", "watching him still as he propped his mirror on the parapet", "dipped the brush in the bowl and lathered cheeks and neck", "Buck Mulligan’s gay voice went on", "My name is absurd too", "Malachi Mulligan", "two dactyls", "But it has a Hellenic ring", "hasn’t it", "Tripping and sunny like the buck himself", "We must go to Athens", "Will you come if I can get the aunt to fork out twenty quid", "He laid the brush aside and", "laughing with delight", "cried", "Will he come", "The jejune jesuit", "Ceasing", "he began to shave with care", "Tell me", "Mulligan", "Stephen said quietly", "Yes", "my love", "How long is Haines going to stay in this tower", "Buck Mulligan showed a shaven cheek over his right shoulder", "God", "isn’t he dreadful", "he said frankly", "A ponderous Saxon", "He thinks you’re not a gentleman", "God", "these bloody English", "Bursting with money and indigestion", "Because he comes from Oxford", "You know", "Dedalus", "you have the real Oxford manner", "He can’t make you out", "O", "my name for you is the best", "Kinch", "the knife-blade", "He shaved warily over his chin", "He was raving all night about a black panther", "Stephen said", "Where is his guncase", "A woful lunatic", "Mulligan said", "Were you in a funk", "I was", "Stephen said with energy and growing fear", "Out here in the dark with a man I don’t know raving and moaning to himself about shooting a black panther", "You saved men from drowning", "I’m not a hero", "however", "If he stays on here I am off", "Buck Mulligan frowned at the lather on his razorblade", "He hopped down from his perch and began to search his trouser pockets hastily", "Scutter", "he cried thickly", "He came over to the gunrest and", "thrusting a hand into Stephen’s upper pocket", "said", "Lend us a loan of your noserag to wipe my razor", "Stephen suffered him to pull out and hold up on show by its corner a dirty crumpled handkerchief", "Buck Mulligan wiped the razorblade neatly", "Then", "gazing over the handkerchief", "he said", "The bard’s noserag", "A new art colour for our Irish poets", "snotgreen", "You can almost taste it", "can’t you", "He mounted to the parapet again and gazed out over Dublin bay", "his fair oakpale hair stirring slightly", "God", "he said quietly", "Isn’t the sea what Algy calls it", "a great sweet mother", "The snotgreen sea", "The scrotumtightening sea", "Epi oinopa ponton", "Ah", "Dedalus", "the Greeks", "I must teach you"]

The first 200 collections:
[["\u{feff}The", "Project", "Gutenberg", "eBook", "of", "Ulysses"], ["by", "James", "Joyce", "This", "eBook", "is", "for", "the", "use", "of", "anyone", "anywhere", "in", "the", "United", "States", "and", "most", "other", "parts", "of", "the", "world", "at", "no", "cost", "and", "with", "almost", "no", "restrictions", "whatsoever"], ["You", "may", "copy", "it"], ["give", "it", "away", "or", "re-use", "it", "under", "the", "terms", "of", "the", "Project", "Gutenberg", "License", "included", "with", "this", "eBook", "or", "online", "at", "www"], ["gutenberg"], ["org"], ["If", "you", "are", "not", "located", "in", "the", "United", "States"], ["you", "will", "have", "to", "check", "the", "laws", "of", "the", "country", "where", "you", "are", "located", "before", "using", "this", "eBook"], ["Title"], ["Ulysses", "Author"], ["James", "Joyce", "Release", "Date"], ["December", "27"], ["2001"], ["eBook", "#4300"], ["Most", "recently", "updated"], ["December", "27"], ["2019"], ["Language"], ["English", "Character", "set", "encoding"], ["UTF-8", "Produced", "by"], ["Col", "Choat", "and", "David", "Widger", "***", "START", "OF", "THE", "PROJECT", "GUTENBERG", "EBOOK", "ULYSSES", "***"], ["Illustration"], ["Ulysses", "by", "James", "Joyce", "Contents"], ["I"], ["1"], ["2"], ["3"], ["II"], ["4"], ["5"], ["6"], ["7"], ["8"], ["9"], ["10"], ["11"], ["12"], ["13"], ["14"], ["15"], ["III"], ["16"], ["17"], ["18"], ["I"], ["1"], ["Stately"], ["plump", "Buck", "Mulligan", "came", "from", "the", "stairhead"], ["bearing", "a", "bowl", "of", "lather", "on", "which", "a", "mirror", "and", "a", "razor", "lay", "crossed"], ["A", "yellow", "dressinggown"], ["ungirdled"], ["was", "sustained", "gently", "behind", "him", "on", "the", "mild", "morning", "air"], ["He", "held", "the", "bowl", "aloft", "and", "intoned"], ["Introibo", "ad", "altare", "Dei"], ["Halted"], ["he", "peered", "down", "the", "dark", "winding", "stairs", "and", "called", "out", "coarsely"], ["Come", "up"], ["Kinch"], ["Come", "up"], ["you", "fearful", "jesuit"], ["Solemnly", "he", "came", "forward", "and", "mounted", "the", "round", "gunrest"], ["He", "faced", "about", "and", "blessed", "gravely", "thrice", "the", "tower"], ["the", "surrounding", "land", "and", "the", "awaking", "mountains"], ["Then"], ["catching", "sight", "of", "Stephen", "Dedalus"], ["he", "bent", "towards", "him", "and", "made", "rapid", "crosses", "in", "the", "air"], ["gurgling", "in", "his", "throat", "and", "shaking", "his", "head"], ["Stephen", "Dedalus"], ["displeased", "and", "sleepy"], ["leaned", "his", "arms", "on", "the", "top", "of", "the", "staircase", "and", "looked", "coldly", "at", "the", "shaking", "gurgling", "face", "that", "blessed", "him"], ["equine", "in", "its", "length"], ["and", "at", "the", "light", "untonsured", "hair"], ["grained", "and", "hued", "like", "pale", "oak"], ["Buck", "Mulligan", "peeped", "an", "instant", "under", "the", "mirror", "and", "then", "covered", "the", "bowl", "smartly"], ["Back", "to", "barracks"], ["he", "said", "sternly"], ["He", "added", "in", "a", "preacher’s", "tone"], ["For", "this"], ["O", "dearly", "beloved"], ["is", "the", "genuine", "Christine"], ["body", "and", "soul", "and", "blood", "and", "ouns"], ["Slow", "music"], ["please"], ["Shut", "your", "eyes"], ["gents"], ["One", "moment"], ["A", "little", "trouble", "about", "those", "white", "corpuscles"], ["Silence"], ["all"], ["He", "peered", "sideways", "up", "and", "gave", "a", "long", "slow", "whistle", "of", "call"], ["then", "paused", "awhile", "in", "rapt", "attention"], ["his", "even", "white", "teeth", "glistening", "here", "and", "there", "with", "gold", "points"], ["Chrysostomos"], ["Two", "strong", "shrill", "whistles", "answered", "through", "the", "calm"], ["Thanks"], ["old", "chap"], ["he", "cried", "briskly"], ["That", "will", "do", "nicely"], ["Switch", "off", "the", "current"], ["will", "you"], ["He", "skipped", "off", "the", "gunrest", "and", "looked", "gravely", "at", "his", "watcher"], ["gathering", "about", "his", "legs", "the", "loose", "folds", "of", "his", "gown"], ["The", "plump", "shadowed", "face", "and", "sullen", "oval", "jowl", "recalled", "a", "prelate"], ["patron", "of", "arts", "in", "the", "middle", "ages"], ["A", "pleasant", "smile", "broke", "quietly", "over", "his", "lips"], ["The", "mockery", "of", "it"], ["he", "said", "gaily"], ["Your", "absurd", "name"], ["an", "ancient", "Greek"], ["He", "pointed", "his", "finger", "in", "friendly", "jest", "and", "went", "over", "to", "the", "parapet"], ["laughing", "to", "himself"], ["Stephen", "Dedalus", "stepped", "up"], ["followed", "him", "wearily", "halfway", "and", "sat", "down", "on", "the", "edge", "of", "the", "gunrest"], ["watching", "him", "still", "as", "he", "propped", "his", "mirror", "on", "the", "parapet"], ["dipped", "the", "brush", "in", "the", "bowl", "and", "lathered", "cheeks", "and", "neck"], ["Buck", "Mulligan’s", "gay", "voice", "went", "on"], ["My", "name", "is", "absurd", "too"], ["Malachi", "Mulligan"], ["two", "dactyls"], ["But", "it", "has", "a", "Hellenic", "ring"], ["hasn’t", "it"], ["Tripping", "and", "sunny", "like", "the", "buck", "himself"], ["We", "must", "go", "to", "Athens"], ["Will", "you", "come", "if", "I", "can", "get", "the", "aunt", "to", "fork", "out", "twenty", "quid"], ["He", "laid", "the", "brush", "aside", "and"], ["laughing", "with", "delight"], ["cried"], ["Will", "he", "come"], ["The", "jejune", "jesuit"], ["Ceasing"], ["he", "began", "to", "shave", "with", "care"], ["Tell", "me"], ["Mulligan"], ["Stephen", "said", "quietly"], ["Yes"], ["my", "love"], ["How", "long", "is", "Haines", "going", "to", "stay", "in", "this", "tower"], ["Buck", "Mulligan", "showed", "a", "shaven", "cheek", "over", "his", "right", "shoulder"], ["God"], ["isn’t", "he", "dreadful"], ["he", "said", "frankly"], ["A", "ponderous", "Saxon"], ["He", "thinks", "you’re", "not", "a", "gentleman"], ["God"], ["these", "bloody", "English"], ["Bursting", "with", "money", "and", "indigestion"], ["Because", "he", "comes", "from", "Oxford"], ["You", "know"], ["Dedalus"], ["you", "have", "the", "real", "Oxford", "manner"], ["He", "can’t", "make", "you", "out"], ["O"], ["my", "name", "for", "you", "is", "the", "best"], ["Kinch"], ["the", "knife-blade"], ["He", "shaved", "warily", "over", "his", "chin"], ["He", "was", "raving", "all", "night", "about", "a", "black", "panther"], ["Stephen", "said"], ["Where", "is", "his", "guncase"], ["A", "woful", "lunatic"], ["Mulligan", "said"], ["Were", "you", "in", "a", "funk"], ["I", "was"], ["Stephen", "said", "with", "energy", "and", "growing", "fear"], ["Out", "here", "in", "the", "dark", "with", "a", "man", "I", "don’t", "know", "raving", "and", "moaning", "to", "himself", "about", "shooting", "a", "black", "panther"], ["You", "saved", "men", "from", "drowning"], ["I’m", "not", "a", "hero"], ["however"], ["If", "he", "stays", "on", "here", "I", "am", "off"], ["Buck", "Mulligan", "frowned", "at", "the", "lather", "on", "his", "razorblade"], ["He", "hopped", "down", "from", "his", "perch", "and", "began", "to", "search", "his", "trouser", "pockets", "hastily"], ["Scutter"], ["he", "cried", "thickly"], ["He", "came", "over", "to", "the", "gunrest", "and"], ["thrusting", "a", "hand", "into", "Stephen’s", "upper", "pocket"], ["said"], ["Lend", "us", "a", "loan", "of", "your", "noserag", "to", "wipe", "my", "razor"], ["Stephen", "suffered", "him", "to", "pull", "out", "and", "hold", "up", "on", "show", "by", "its", "corner", "a", "dirty", "crumpled", "handkerchief"], ["Buck", "Mulligan", "wiped", "the", "razorblade", "neatly"], ["Then"], ["gazing", "over", "the", "handkerchief"], ["he", "said"], ["The", "bard’s", "noserag"], ["A", "new", "art", "colour", "for", "our", "Irish", "poets"], ["snotgreen"], ["You", "can", "almost", "taste", "it"], ["can’t", "you"], ["He", "mounted", "to", "the", "parapet", "again", "and", "gazed", "out", "over", "Dublin", "bay"], ["his", "fair", "oakpale", "hair", "stirring", "slightly"], ["God"], ["he", "said", "quietly"], ["Isn’t", "the", "sea", "what", "Algy", "calls", "it"], ["a", "great", "sweet", "mother"], ["The", "snotgreen", "sea"], ["The", "scrotumtightening", "sea"], ["Epi", "oinopa", "ponton"], ["Ah"], ["Dedalus"], ["the", "Greeks"], ["I", "must", "teach", "you"]]

The number of words :34816
The ordered (by frequency) vocab of words (200 words):
[("the", 13676), ("of", 8171), ("and", 6695), ("a", 5885), ("to", 4873), ("in", 4726), ("his", 3063), ("he", 2880), ("I", 2683), ("that", 2415), ("with", 2414), ("it", 2145), ("was", 2094), ("on", 2025), ("for", 1836), ("her", 1686), ("you", 1674), ("him", 1507), ("is", 1372), ("at", 1232), ("The", 1224), ("by", 1213), ("said", 1202), ("He", 1155), ("all", 1154), ("as", 1115), ("from", 1027), ("or", 959), ("me", 922), ("be", 875), ("out", 873), ("up", 812), ("she", 810), ("not", 807), ("had", 789), ("they", 780), ("Mr", 717), ("my", 715), ("A", 687), ("their", 681), ("like", 671), ("them", 668), ("have", 652), ("Bloom", 641), ("one", 614), ("an", 594), ("there", 589), ("And", 574), ("about", 529), ("so", 514), ("are", 503), ("which", 500), ("were", 492), ("when", 491), ("what", 487), ("says", 469), ("your", 456), ("if", 449), ("but", 441), ("old", 437), ("down", 432), ("over", 429), ("then", 428), ("this", 421), ("Stephen", 414), ("no", 413), ("too", 413), ("What", 408), ("man", 394), ("now", 381), ("see", 381), ("after", 374), ("who", 368), ("O", 366), ("do", 365), ("would", 362), ("off", 356), ("back", 354), ("time", 353), ("did", 335), ("two", 330), ("into", 326), ("She", 325), ("eyes", 320), ("will", 318), ("other", 315), ("know", 313), ("hand", 300), ("its", 299), ("we", 295), ("could", 293), ("more", 288), ("some", 288), ("those", 286), ("You", 284), ("street", 282), ("BLOOM", 280), ("way", 270), ("has", 269), ("little", 268), ("His", 266), ("In", 259), ("But", 258), ("us", 250), ("good", 242), ("No", 241), ("They", 240), ("say", 240), ("our", 238), ("through", 237), ("again", 235), ("day", 235), ("face", 231), ("get", 229), ("first", 228), ("round", 226), ("It", 223), ("THE", 222), ("long", 220), ("only", 220), ("here", 217), ("right", 216), ("where", 216), ("night", 215), ("himself", 214), ("under", 214), ("any", 213), ("name", 213), ("never", 212), ("head", 209), ("before", 206), ("go", 206), ("sir", 204), ("J", 203), ("been", 203), ("That", 202), ("God", 199), ("put", 199), ("Yes", 198), ("very", 198), ("thing", 196), ("came", 193), ("going", 191), ("just", 191), ("can", 189), ("John", 187), ("asked", 186), ("life", 182), ("away", 181), ("All", 180), ("went", 179), ("come", 178), ("made", 177), ("young", 177), ("because", 175), ("well", 175), ("being", 171), ("than", 171), ("woman", 170), ("Dedalus", 165), ("Mrs", 165), ("To", 165), ("hat", 165), ("last", 164), ("left", 163), ("much", 162), ("house", 161), ("must", 161), ("yes", 161), ("make", 160), ("ever", 159), ("though", 158), ("voice", 158), ("course", 157), ("always", 156), ("saw", 155), ("something", 154), ("got", 153), ("look", 153), ("I’m", 152), ("same", 152), ("Then", 151), ("white", 151), ("might", 150), ("own", 150), ("Mulligan", 148), ("hands", 148), ("world", 147), ("am", 146), ("With", 144), ("poor", 142), ("told", 142), ("new", 141), ("without", 141), ("behind", 139), ("father", 139), ("took", 139), ("How", 138), ("let", 138), ("We", 137)]

0 the
1 of
2 and
3 a
4 to
5 in
6 his
7 he
8 I
9 that
10 with
11 it
12 was
13 on
14 for
15 her
16 you
17 him
18 is
19 at
20 The
21 by
22 said
23 He
24 all
25 as
26 from
27 or
28 me
29 be
30 out
31 up
32 she
33 not
34 had
35 they
36 Mr
37 my
38 A
39 their
40 like
41 them
42 have
43 Bloom
44 one
45 an
46 there
47 And
48 about
49 so
50 are
51 which
52 were
53 when
54 what
55 says
56 your
57 if
58 but
59 old
60 down
61 over
62 then
63 this
64 Stephen
65 no
66 too
67 What
68 man
69 now
70 see
71 after
72 who
73 O
74 do
75 would
76 off
77 back
78 time
79 did
80 two
81 into
82 She
83 eyes
84 will
85 other
86 know
87 hand
88 its
89 we
90 could
91 more
92 some
93 those
94 You
95 street
96 BLOOM
97 way
98 has
99 little
100 His
101 In
102 But
103 us
104 good
105 No
106 They
107 say
108 our
109 through
110 again
111 day
112 face
113 get
114 first
115 round
116 It
117 THE
118 long
119 only
120 here
121 right
122 where
123 night
124 himself
125 under
126 any
127 name
128 never
129 head
130 before
131 go
132 sir
133 J
134 been
135 That
136 God
137 put
138 Yes
139 very
140 thing
141 came
142 going
143 just
144 can
145 John
146 asked
147 life
148 away
149 All
150 went
151 come
152 made
153 young
154 because
155 well
156 being
157 than
158 woman
159 Dedalus
160 Mrs
161 To
162 hat
163 last
164 left
165 much
166 house
167 must
168 yes
169 make
170 ever
171 though
172 voice
173 course
174 always
175 saw
176 something
177 got
178 look
179 I’m
180 same
181 Then
182 white
183 might
184 own
185 Mulligan
186 hands
187 world
188 am
189 With
190 poor
191 told
192 new
193 without
194 behind
195 father
196 took
197 How
198 let
199 We
200 door
The vocab of index_of_word -> quantity of the word in vocab (for 200 words):
[(0, 13676), (1, 8171), (2, 6695), (3, 5885), (4, 4873), (5, 4726), (6, 3063), (7, 2880), (8, 2683), (9, 2415), (10, 2414), (11, 2145), (12, 2094), (13, 2025), (14, 1836), (15, 1686), (16, 1674), (17, 1507), (18, 1372), (19, 1232), (20, 1224), (21, 1213), (22, 1202), (23, 1155), (24, 1154), (25, 1115), (26, 1027), (27, 959), (28, 922), (29, 875), (30, 873), (31, 812), (32, 810), (33, 807), (34, 789), (35, 780), (36, 717), (37, 715), (38, 687), (39, 681), (40, 671), (41, 668), (42, 652), (43, 641), (44, 614), (45, 594), (46, 589), (47, 574), (48, 529), (49, 514), (50, 503), (51, 500), (52, 492), (53, 491), (54, 487), (55, 469), (56, 456), (57, 449), (58, 441), (59, 437), (60, 432), (61, 429), (62, 428), (63, 421), (64, 414), (65, 413), (66, 413), (67, 408), (68, 394), (69, 381), (70, 381), (71, 374), (72, 368), (73, 366), (74, 365), (75, 362), (76, 356), (77, 354), (78, 353), (79, 335), (80, 330), (81, 326), (82, 325), (83, 320), (84, 318), (85, 315), (86, 313), (87, 300), (88, 299), (89, 295), (90, 293), (91, 288), (92, 288), (93, 286), (94, 284), (95, 282), (96, 280), (97, 270), (98, 269), (99, 268), (100, 266), (101, 259), (102, 258), (103, 250), (104, 242), (105, 241), (106, 240), (107, 240), (108, 238), (109, 237), (110, 235), (111, 235), (112, 231), (113, 229), (114, 228), (115, 226), (116, 223), (117, 222), (118, 220), (119, 220), (120, 217), (121, 216), (122, 216), (123, 215), (124, 214), (125, 214), (126, 213), (127, 213), (128, 212), (129, 209), (130, 206), (131, 206), (132, 204), (133, 203), (134, 203), (135, 202), (136, 199), (137, 199), (138, 198), (139, 198), (140, 196), (141, 193), (142, 191), (143, 191), (144, 189), (145, 187), (146, 186), (147, 182), (148, 181), (149, 180), (150, 179), (151, 178), (152, 177), (153, 177), (154, 175), (155, 175), (156, 171), (157, 171), (158, 170), (159, 165), (160, 165), (161, 165), (162, 165), (163, 164), (164, 163), (165, 162), (166, 161), (167, 161), (168, 161), (169, 160), (170, 159), (171, 158), (172, 158), (173, 157), (174, 156), (175, 155), (176, 154), (177, 153), (178, 153), (179, 152), (180, 152), (181, 151), (182, 151), (183, 150), (184, 150), (185, 148), (186, 148), (187, 147), (188, 146), (189, 144), (190, 142), (191, 142), (192, 141), (193, 141), (194, 139), (195, 139), (196, 139), (197, 138), (198, 138), (199, 137)]
The collection of sentences as indices (for 200 words):
[[34815, 326, 933, 2351, 1, 3524], [21, 872, 7332, 395, 2351, 18, 14, 0, 590, 1, 1351, 2154, 5, 0, 1238, 1748, 2, 211, 85, 1263, 1, 0, 187, 19, 65, 2337, 2, 10, 1012, 65, 13824, 3934], [94, 361, 1293, 11], [226, 11, 148, 27, 13716, 11, 125, 0, 794, 1, 0, 326, 933, 2493, 2194, 10, 63, 2351, 27, 6577, 19, 3030], [2593], [5256], [241, 16, 50, 33, 2613, 5, 0, 1238, 1748], [16, 84, 42, 4, 4931, 0, 1688, 1, 0, 482, 122, 16, 50, 2613, 130, 3404, 63, 2351], [20590], [3524, 9567], [872, 7332, 19627, 16450], [3449, 3037], [9481], [2351, 14899], [1177, 3832, 34004], [3449, 3037], [14961], [5794], [968, 15995, 399, 25119], [20748, 19441, 21], [16148, 16046, 2, 1973, 21066, 5589, 10596, 768, 117, 5842, 7255, 9888, 10814, 5589], [4721], [3524, 21, 872, 7332, 16234], [8], [240], [442], [695], [5764], [869], [746], [1402], [1473], [1338], [1127], [1472], [2107], [3033], [3423], [2720], [1554], [5765], [1962], [3424], [4625], [8], [240], [20260], [1445, 274, 185, 141, 26, 0, 9149], [1096, 3, 1884, 1, 6447, 13, 51, 3, 647, 2, 3, 3828, 433, 1064], [38, 592, 24817], [14639], [12, 2449, 773, 194, 17, 13, 0, 2618, 249, 251], [23, 439, 0, 1884, 5954, 2, 27584], [10149, 819, 11014, 9829], [10072], [7, 2225, 60, 0, 234, 2716, 1544, 2, 314, 30, 23412], [301, 31], [908], [301, 31], [16, 12169, 2896], [7539, 7, 141, 370, 2, 6511, 0, 115, 6347], [23, 6263, 48, 2, 1016, 1374, 9246, 0, 1170], [0, 14370, 341, 2, 0, 11123, 3761], [181], [23048, 882, 1, 64, 159], [7, 728, 235, 17, 2, 152, 5339, 8022, 5, 0, 251], [6348, 5, 6, 1465, 2, 2423, 6, 129], [64, 159], [11894, 2, 6814], [1918, 6, 352, 13, 0, 867, 1, 0, 1628, 2, 268, 11535, 19, 0, 2423, 6348, 112, 9, 1016, 17], [6240, 5, 88, 2609], [2, 19, 0, 272, 33988, 239], [26438, 2, 27102, 40, 1528, 5247], [274, 185, 8744, 45, 755, 125, 0, 647, 2, 62, 1428, 0, 1884, 3877], [3043, 4, 4876], [7, 22, 3884], [23, 770, 5, 3, 30342, 1331], [231, 63], [73, 11784, 4887], [18, 0, 3689, 16062], [397, 2, 410, 2, 539, 2, 13333], [20122, 408], [782], [3511, 56, 83], [6318], [309, 324], [38, 99, 834, 48, 93, 182, 23841], [3512], [24], [23, 2225, 1213, 31, 2, 233, 3, 118, 789, 1959, 1, 283], [62, 4437, 1659, 5, 6699, 910], [6, 259, 182, 608, 4310, 120, 2, 46, 10, 359, 783], [9738], [453, 1000, 3871, 14811, 414, 109, 0, 2326], [1237], [59, 559], [7, 340, 3581], [135, 84, 74, 3767], [20407, 76, 0, 4224], [84, 16], [23, 14110, 76, 0, 6347, 2, 268, 1374, 19, 6, 9402], [3234, 48, 6, 949, 0, 855, 4294, 1, 6, 2369], [20, 1445, 9048, 112, 2, 14340, 3779, 12811, 30911, 3, 13575], [5274, 1, 3553, 5, 0, 898, 1657], [38, 1531, 652, 1019, 989, 61, 6, 320], [20, 2919, 1, 11], [7, 22, 2870], [681, 4828, 127], [45, 845, 1279], [23, 1817, 6, 731, 5, 2586, 5152, 2, 150, 61, 4, 0, 2928], [562, 4, 124], [64, 159, 1632, 31], [512, 17, 14773, 12477, 2, 674, 60, 13, 0, 1900, 1, 0, 6347], [1400, 17, 214, 25, 7, 5327, 6, 647, 13, 0, 2928], [5005, 0, 2324, 5, 0, 1884, 2, 12868, 1427, 2, 585], [274, 2135, 1373, 172, 150, 13], [229, 127, 18, 4828, 66], [1132, 185], [80, 24135], [102, 11, 98, 3, 10091, 863], [6350, 11], [20678, 2, 6870, 40, 0, 2547, 124], [199, 167, 131, 4, 15294], [751, 16, 151, 57, 8, 144, 113, 0, 1352, 4, 2039, 30, 1399, 1211], [23, 524, 0, 2324, 890, 2], [562, 10, 2030], [340], [751, 7, 151], [20, 27706, 2896], [15961], [7, 538, 4, 3866, 10, 627], [700, 28], [185], [64, 22, 989], [138], [37, 218], [197, 118, 18, 432, 142, 4, 2094, 5, 63, 1170], [274, 185, 1395, 3, 5413, 1426, 61, 6, 121, 881], [136], [1797, 7, 3638], [7, 22, 4297], [38, 6648, 4062], [23, 1166, 623, 33, 3, 323], [136], [297, 277, 968], [15800, 10, 279, 2, 27402], [598, 7, 461, 26, 3097], [94, 86], [159], [16, 42, 0, 879, 3097, 1152], [23, 613, 169, 16, 30], [73], [37, 127, 14, 16, 18, 0, 311], [908], [0, 27869], [23, 6791, 6993, 61, 6, 2331], [23, 12, 8904, 24, 123, 48, 3, 221, 4431], [64, 22], [291, 18, 6, 26593], [38, 14850, 5184], [185, 22], [1418, 16, 5, 3, 12299], [8, 12], [64, 22, 10, 5032, 2, 3699, 772], [1089, 120, 5, 0, 234, 10, 3, 68, 8, 205, 86, 8904, 2, 6503, 4, 124, 48, 3870, 3, 221, 4431], [94, 1622, 206, 26, 2842], [179, 33, 3, 1681], [685], [241, 7, 2686, 13, 120, 8, 188, 76], [274, 185, 2867, 19, 0, 6447, 13, 6, 8905], [23, 27013, 60, 26, 6, 6611, 2, 538, 4, 2421, 6, 5529, 1035, 6351], [19930], [7, 340, 6907], [23, 141, 61, 4, 0, 6347, 2], [14480, 3, 87, 81, 403, 1468, 366], [22], [7359, 103, 3, 12939, 1, 56, 13259, 4, 4612, 37, 3828], [64, 3380, 17, 4, 1706, 30, 2, 735, 31, 13, 440, 21, 88, 315, 3, 1368, 3617, 1147], [274, 185, 3026, 0, 8905, 3299], [181], [4308, 61, 0, 1147], [7, 22], [20, 11158, 13259], [38, 192, 625, 1186, 14, 108, 245, 3802], [14153], [94, 144, 1012, 1634, 11], [613, 16], [23, 6511, 4, 0, 2928, 110, 2, 1107, 30, 61, 248, 1181], [6, 540, 29282, 239, 6854, 1538], [136], [7, 22, 989], [1981, 0, 307, 54, 9529, 705, 11], [3, 278, 436, 288], [20, 14153, 307], [20, 31624, 307], [9923, 8692, 8795], [384], [159], [0, 3465], [8, 167, 3388, 16]]
The max pairs :(([1], [0]), 1640)
Iteration number:========== 0
Max pair !!!========== (([1], [0]), 1640)
Max_pair_as_words ========== (["of"], ["the"])

Iteration number:========== 1
Max pair !!!========== (([5], [0]), 1343)
Max_pair_as_words ========== (["in"], ["the"])

Iteration number:========== 2
Max pair !!!========== (([13], [0]), 606)
Max_pair_as_words ========== (["on"], ["the"])

Iteration number:========== 3
Max pair !!!========== (([4], [0]), 561)
Max_pair_as_words ========== (["to"], ["the"])

Iteration number:========== 4
Max pair !!!========== (([2], [0]), 362)
Max_pair_as_words ========== (["and"], ["the"])

Iteration number:========== 5
Max pair !!!========== (([1], [3]), 319)
Max_pair_as_words ========== (["of"], ["a"])

Iteration number:========== 6
Max pair !!!========== (([14], [0]), 290)
Max_pair_as_words ========== (["for"], ["the"])

Iteration number:========== 7
Max pair !!!========== (([19], [0]), 288)
Max_pair_as_words ========== (["at"], ["the"])

Iteration number:========== 8
Max pair !!!========== (([26], [0]), 284)
Max_pair_as_words ========== (["from"], ["the"])

Iteration number:========== 9
Max pair !!!========== (([5], [3]), 271)
Max_pair_as_words ========== (["in"], ["a"])

Iteration number:========== 10
Max pair !!!========== (([10], [3]), 245)
Max_pair_as_words ========== (["with"], ["a"])

Iteration number:========== 11
Max pair !!!========== (([21], [0]), 232)
Max_pair_as_words ========== (["by"], ["the"])

Iteration number:========== 12
Max pair !!!========== (([7], [22]), 230)
Max_pair_as_words ========== (["he"], ["said"])

Iteration number:========== 13
Max pair !!!========== (([10], [0]), 223)
Max_pair_as_words ========== (["with"], ["the"])

Iteration number:========== 14
Max pair !!!========== (([4], [29]), 220)
Max_pair_as_words ========== (["to"], ["be"])

Iteration number:========== 15
Max pair !!!========== (([36], [43]), 215)
Max_pair_as_words ========== (["Mr"], ["Bloom"])

Iteration number:========== 16
Max pair !!!========== (([5], [6]), 211)
Max_pair_as_words ========== (["in"], ["his"])

Iteration number:========== 17
Max pair !!!========== (([1], [6]), 199)
Max_pair_as_words ========== (["of"], ["his"])

Iteration number:========== 18
Max pair !!!========== (([7], [12]), 169)
Max_pair_as_words ========== (["he"], ["was"])

Iteration number:========== 19
Max pair !!!========== (([7], [34]), 141)
Max_pair_as_words ========== (["he"], ["had"])

Iteration number:========== 20
Max pair !!!========== (([30], [1]), 139)
Max_pair_as_words ========== (["out"], ["of"])

Iteration number:========== 21
Max pair !!!========== (([11], [12]), 134)
Max_pair_as_words ========== (["it"], ["was"])

Iteration number:========== 22
Max pair !!!========== (([14], [3]), 128)
Max_pair_as_words ========== (["for"], ["a"])

Iteration number:========== 23
Max pair !!!========== (([2], [3]), 119)
Max_pair_as_words ========== (["and"], ["a"])

Iteration number:========== 24
Max pair !!!========== (([13], [6]), 118)
Max_pair_as_words ========== (["on"], ["his"])

Iteration number:========== 25
Max pair !!!========== (([8], [12]), 114)
Max_pair_as_words ========== (["I"], ["was"])

Iteration number:========== 26
Max pair !!!========== (([8], [188]), 112)
Max_pair_as_words ========== (["I"], ["am"])

Iteration number:========== 27
Max pair !!!========== (([24], [0]), 110)
Max_pair_as_words ========== (["all"], ["the"])

Iteration number:========== 28
Max pair !!!========== (([4], [6]), 105)
Max_pair_as_words ========== (["to"], ["his"])

Iteration number:========== 29
Max pair !!!========== (([64], [22]), 104)
Max_pair_as_words ========== (["Stephen"], ["said"])

Iteration number:========== 30
Max pair !!!========== (([18], [0]), 104)
Max_pair_as_words ========== (["is"], ["the"])

Iteration number:========== 31
Max pair !!!========== (([36], [159]), 103)
Max_pair_as_words ========== (["Mr"], ["Dedalus"])

Iteration number:========== 32
Max pair !!!========== (([10], [6]), 101)
Max_pair_as_words ========== (["with"], ["his"])

Iteration number:========== 33
Max pair !!!========== (([12], [3]), 94)
Max_pair_as_words ========== (["was"], ["a"])

Iteration number:========== 34
Max pair !!!========== (([274], [185]), 91)
Max_pair_as_words ========== (["Buck"], ["Mulligan"])

Iteration number:========== 35
Max pair !!!========== (([4], [3]), 86)
Max_pair_as_words ========== (["to"], ["a"])

Iteration number:========== 36
Max pair !!!========== (([1], [15]), 86)
Max_pair_as_words ========== (["of"], ["her"])

Iteration number:========== 37
Max pair !!!========== (([61], [0]), 85)
Max_pair_as_words ========== (["over"], ["the"])

Iteration number:========== 38
Max pair !!!========== (([55], [203]), 85)
Max_pair_as_words ========== (["says"], ["Joe"])

Iteration number:========== 39
Max pair !!!========== (([13], [3]), 84)
Max_pair_as_words ========== (["on"], ["a"])

Iteration number:========== 40
Max pair !!!========== (([109], [0]), 84)
Max_pair_as_words ========== (["through"], ["the"])

Iteration number:========== 41
Max pair !!!========== (([12], [0]), 82)
Max_pair_as_words ========== (["was"], ["the"])

Iteration number:========== 42
Max pair !!!========== (([25], [3]), 79)
Max_pair_as_words ========== (["as"], ["a"])

Iteration number:========== 43
Max pair !!!========== (([5], [15]), 76)
Max_pair_as_words ========== (["in"], ["her"])

Iteration number:========== 44
Max pair !!!========== (([55], [7]), 75)
Max_pair_as_words ========== (["says"], ["he"])

Iteration number:========== 45
Max pair !!!========== (([81], [0]), 74)
Max_pair_as_words ========== (["into"], ["the"])

Iteration number:========== 46
Max pair !!!========== (([18], [3]), 74)
Max_pair_as_words ========== (["is"], ["a"])

Iteration number:========== 47
Max pair !!!========== (([8], [42]), 71)
Max_pair_as_words ========== (["I"], ["have"])

Iteration number:========== 48
Max pair !!!========== (([2], [210]), 70)
Max_pair_as_words ========== (["and"], ["S"])

Iteration number:========== 49
Max pair !!!========== (([40], [9]), 70)
Max_pair_as_words ========== (["like"], ["that"])

Iteration number:========== 50
Max pair !!!========== (([11], [18]), 70)
Max_pair_as_words ========== (["it"], ["is"])

Iteration number:========== 51
Max pair !!!========== (([25], [0]), 67)
Max_pair_as_words ========== (["as"], ["the"])

Iteration number:========== 52
Max pair !!!========== (([269], [376]), 67)
Max_pair_as_words ========== (["Martin"], ["Cunningham"])

Iteration number:========== 53
Max pair !!!========== (([25], [7]), 67)
Max_pair_as_words ========== (["as"], ["he"])

Iteration number:========== 54
Max pair !!!========== (([16], [86]), 65)
Max_pair_as_words ========== (["you"], ["know"])

Iteration number:========== 55
Max pair !!!========== (([4], [17]), 64)
Max_pair_as_words ========== (["to"], ["him"])

Iteration number:========== 56
Max pair !!!========== (([0], [305]), 63)
Max_pair_as_words ========== (["the"], ["citizen"])

Iteration number:========== 57
Max pair !!!========== (([26], [6]), 63)
Max_pair_as_words ========== (["from"], ["his"])

Iteration number:========== 58
Max pair !!!========== (([40], [3]), 62)
Max_pair_as_words ========== (["like"], ["a"])

Iteration number:========== 59
Max pair !!!========== (([5], [39]), 62)
Max_pair_as_words ========== (["in"], ["their"])

Iteration number:========== 60
Max pair !!!========== (([9], [12]), 61)
Max_pair_as_words ========== (["that"], ["was"])

Iteration number:========== 61
Max pair !!!========== (([18], [11]), 57)
Max_pair_as_words ========== (["is"], ["it"])

Iteration number:========== 62
Max pair !!!========== (([48], [0]), 57)
Max_pair_as_words ========== (["about"], ["the"])

Iteration number:========== 63
Max pair !!!========== (([4], [15]), 57)
Max_pair_as_words ========== (["to"], ["her"])

Iteration number:========== 64
Max pair !!!========== (([4], [70]), 55)
Max_pair_as_words ========== (["to"], ["see"])

Iteration number:========== 65
Max pair !!!========== (([21], [3]), 54)
Max_pair_as_words ========== (["by"], ["a"])

Iteration number:========== 66
Max pair !!!========== (([101], [0]), 54)
Max_pair_as_words ========== (["In"], ["the"])

Iteration number:========== 67
Max pair !!!========== (([125], [0]), 54)
Max_pair_as_words ========== (["under"], ["the"])

Iteration number:========== 68
Max pair !!!========== (([1], [24]), 53)
Max_pair_as_words ========== (["of"], ["all"])

Iteration number:========== 69
Max pair !!!========== (([7], [18]), 53)
Max_pair_as_words ========== (["he"], ["is"])

Iteration number:========== 70
Max pair !!!========== (([1], [9]), 53)
Max_pair_as_words ========== (["of"], ["that"])

Iteration number:========== 71
Max pair !!!========== (([32], [12]), 53)
Max_pair_as_words ========== (["she"], ["was"])

Iteration number:========== 72
Max pair !!!========== (([55], [0, 305]), 52)
Max_pair_as_words ========== (["says"], ["the", "citizen"])

Iteration number:========== 73
Max pair !!!========== (([47], [0]), 52)
Max_pair_as_words ========== (["And"], ["the"])

Iteration number:========== 74
Max pair !!!========== (([1], [173]), 51)
Max_pair_as_words ========== (["of"], ["course"])

Iteration number:========== 75
Max pair !!!========== (([8], [86]), 51)
Max_pair_as_words ========== (["I"], ["know"])

Iteration number:========== 76
Max pair !!!========== (([1], [41]), 51)
Max_pair_as_words ========== (["of"], ["them"])

Iteration number:========== 77
Max pair !!!========== (([2], [7]), 50)
Max_pair_as_words ========== (["and"], ["he"])

Iteration number:========== 78
Max pair !!!========== (([208], [388]), 49)
Max_pair_as_words ========== (["Father"], ["Conmee"])

Iteration number:========== 79
Max pair !!!========== (([13], [15]), 49)
Max_pair_as_words ========== (["on"], ["her"])

Iteration number:========== 80
Max pair !!!========== (([235], [0]), 48)
Max_pair_as_words ========== (["towards"], ["the"])

Iteration number:========== 81
Max pair !!!========== (([4], [107]), 48)
Max_pair_as_words ========== (["to"], ["say"])

Iteration number:========== 82
Max pair !!!========== (([75], [29]), 47)
Max_pair_as_words ========== (["would"], ["be"])

Iteration number:========== 83
Max pair !!!========== (([60], [0]), 47)
Max_pair_as_words ========== (["down"], ["the"])

Iteration number:========== 84
Max pair !!!========== (([31], [0]), 47)
Max_pair_as_words ========== (["up"], ["the"])

Iteration number:========== 85
Max pair !!!========== (([74], [16]), 47)
Max_pair_as_words ========== (["do"], ["you"])

Iteration number:========== 86
Max pair !!!========== (([1], [11]), 47)
Max_pair_as_words ========== (["of"], ["it"])

Iteration number:========== 87
Max pair !!!========== (([0], [85]), 46)
Max_pair_as_words ========== (["the"], ["other"])

Iteration number:========== 88
Max pair !!!========== (([19], [6]), 45)
Max_pair_as_words ========== (["at"], ["his"])

Iteration number:========== 89
Max pair !!!========== (([32], [22]), 45)
Max_pair_as_words ========== (["she"], ["said"])

Iteration number:========== 90
Max pair !!!========== (([116], [12]), 45)
Max_pair_as_words ========== (["It"], ["was"])

Iteration number:========== 91
Max pair !!!========== (([508], [16]), 45)
Max_pair_as_words ========== (["Are"], ["you"])

Iteration number:========== 92
Max pair !!!========== (([9], [18]), 45)
Max_pair_as_words ========== (["that"], ["is"])

Iteration number:========== 93
Max pair !!!========== (([57], [16]), 44)
Max_pair_as_words ========== (["if"], ["you"])

Iteration number:========== 94
Max pair !!!========== (([55], [8]), 43)
Max_pair_as_words ========== (["says"], ["I"])

Iteration number:========== 95
Max pair !!!========== (([18], [9]), 43)
Max_pair_as_words ========== (["is"], ["that"])

Iteration number:========== 96
Max pair !!!========== (([9], [0]), 42)
Max_pair_as_words ========== (["that"], ["the"])

Iteration number:========== 97
Max pair !!!========== (([313], [16]), 42)
Max_pair_as_words ========== (["Do"], ["you"])

Iteration number:========== 98
Max pair !!!========== (([8], [375]), 42)
Max_pair_as_words ========== (["I"], ["mean"])

Iteration number:========== 99
Max pair !!!========== (([2], [6]), 41)
Max_pair_as_words ========== (["and"], ["his"])

Iteration number:========== 100
Max pair !!!========== (([116], [18]), 41)
Max_pair_as_words ========== (["It"], ["is"])

Iteration number:========== 101
Max pair !!!========== (([32], [34]), 41)
Max_pair_as_words ========== (["she"], ["had"])

Iteration number:========== 102
Max pair !!!========== (([36], [451]), 41)
Max_pair_as_words ========== (["Mr"], ["Power"])

Iteration number:========== 103
Max pair !!!========== (([8], [205]), 40)
Max_pair_as_words ========== (["I"], ["don’t"])

Iteration number:========== 104
Max pair !!!========== (([6], [129]), 40)
Max_pair_as_words ========== (["his"], ["head"])

Iteration number:========== 105
Max pair !!!========== (([8], [84]), 40)
Max_pair_as_words ========== (["I"], ["will"])

Iteration number:========== 106
Max pair !!!========== (([241], [16]), 40)
Max_pair_as_words ========== (["If"], ["you"])

Iteration number:========== 107
Max pair !!!========== (([8], [70]), 40)
Max_pair_as_words ========== (["I"], ["see"])

Iteration number:========== 108
Max pair !!!========== (([3], [104]), 39)
Max_pair_as_words ========== (["a"], ["good"])

Iteration number:========== 109
Max pair !!!========== (([4], [28]), 39)
Max_pair_as_words ========== (["to"], ["me"])

Iteration number:========== 110
Max pair !!!========== (([23], [12]), 39)
Max_pair_as_words ========== (["He"], ["was"])

Iteration number:========== 111
Max pair !!!========== (([402], [568]), 39)
Max_pair_as_words ========== (["Ned"], ["Lambert"])

Iteration number:========== 112
Max pair !!!========== (([4], [42]), 38)
Max_pair_as_words ========== (["to"], ["have"])

Iteration number:========== 113
Max pair !!!========== (([3], [204]), 38)
Max_pair_as_words ========== (["a"], ["bit"])

Iteration number:========== 114
Max pair !!!========== (([14], [15]), 38)
Max_pair_as_words ========== (["for"], ["her"])

Iteration number:========== 115
Max pair !!!========== (([300], [0]), 37)
Max_pair_as_words ========== (["On"], ["the"])

Iteration number:========== 116
Max pair !!!========== (([5], [37]), 37)
Max_pair_as_words ========== (["in"], ["my"])

Iteration number:========== 117
Max pair !!!========== (([55], [443]), 37)
Max_pair_as_words ========== (["says"], ["Alf"])

Iteration number:========== 118
Max pair !!!========== (([11], [31]), 37)
Max_pair_as_words ========== (["it"], ["up"])

Iteration number:========== 119
Max pair !!!========== (([35], [52]), 36)
Max_pair_as_words ========== (["they"], ["were"])

Iteration number:========== 120
Max pair !!!========== (([1], [56]), 36)
Max_pair_as_words ========== (["of"], ["your"])

Iteration number:========== 121
Max pair !!!========== (([5], [9]), 36)
Max_pair_as_words ========== (["in"], ["that"])

Iteration number:========== 122
Max pair !!!========== (([26], [3]), 36)
Max_pair_as_words ========== (["from"], ["a"])

Iteration number:========== 123
Max pair !!!========== (([30], [1, 0]), 36)
Max_pair_as_words ========== (["out"], ["of", "the"])

Iteration number:========== 124
Max pair !!!========== (([458], [458]), 36)
Max_pair_as_words ========== (["*"], ["*"])

Iteration number:========== 125
Max pair !!!========== (([14], [17]), 36)
Max_pair_as_words ========== (["for"], ["him"])

Iteration number:========== 126
Max pair !!!========== (([36], [659]), 35)
Max_pair_as_words ========== (["Mr"], ["Deasy"])

Iteration number:========== 127
Max pair !!!========== (([34], [134]), 35)
Max_pair_as_words ========== (["had"], ["been"])

Iteration number:========== 128
Max pair !!!========== (([4], [113]), 35)
Max_pair_as_words ========== (["to"], ["get"])

Iteration number:========== 129
Max pair !!!========== (([40], [0]), 35)
Max_pair_as_words ========== (["like"], ["the"])

Iteration number:========== 130
Max pair !!!========== (([10], [15]), 35)
Max_pair_as_words ========== (["with"], ["her"])

Iteration number:========== 131
Max pair !!!========== (([53], [0]), 34)
Max_pair_as_words ========== (["when"], ["the"])

Iteration number:========== 132
Max pair !!!========== (([50], [16]), 34)
Max_pair_as_words ========== (["are"], ["you"])

Iteration number:========== 133
Max pair !!!========== (([189], [3]), 33)
Max_pair_as_words ========== (["With"], ["a"])

Iteration number:========== 134
Max pair !!!========== (([8], [175]), 33)
Max_pair_as_words ========== (["I"], ["saw"])

Iteration number:========== 135
Max pair !!!========== (([6], [87]), 33)
Max_pair_as_words ========== (["his"], ["hand"])

Iteration number:========== 136
Max pair !!!========== (([84], [29]), 33)
Max_pair_as_words ========== (["will"], ["be"])

Iteration number:========== 137
Max pair !!!========== (([699], [657]), 33)
Max_pair_as_words ========== (["Myles"], ["Crawford"])

Iteration number:========== 138
Max pair !!!========== (([8], [144]), 33)
Max_pair_as_words ========== (["I"], ["can"])

Iteration number:========== 139
Max pair !!!========== (([23], [18]), 32)
Max_pair_as_words ========== (["He"], ["is"])

Iteration number:========== 140
Max pair !!!========== (([65], [91]), 32)
Max_pair_as_words ========== (["no"], ["more"])

Iteration number:========== 141
Max pair !!!========== (([326], [490]), 32)
Max_pair_as_words ========== (["Project"], ["Gutenberg-tm"])

Iteration number:========== 142
Max pair !!!========== (([79], [7]), 32)
Max_pair_as_words ========== (["did"], ["he"])

Iteration number:========== 143
Max pair !!!========== (([79], [33]), 32)
Max_pair_as_words ========== (["did"], ["not"])

Iteration number:========== 144
Max pair !!!========== (([16], [70]), 32)
Max_pair_as_words ========== (["you"], ["see"])

Iteration number:========== 145
Max pair !!!========== (([115], [0]), 32)
Max_pair_as_words ========== (["round"], ["the"])

Iteration number:========== 146
Max pair !!!========== (([14], [9]), 31)
Max_pair_as_words ========== (["for"], ["that"])

Iteration number:========== 147
Max pair !!!========== (([14], [16]), 31)
Max_pair_as_words ========== (["for"], ["you"])

Iteration number:========== 148
Max pair !!!========== (([1], [39]), 31)
Max_pair_as_words ========== (["of"], ["their"])

Iteration number:========== 149
Max pair !!!========== (([8], [207]), 31)
Max_pair_as_words ========== (["I"], ["think"])

Iteration number:========== 150
Max pair !!!========== (([75], [42]), 30)
Max_pair_as_words ========== (["would"], ["have"])

Iteration number:========== 151
Max pair !!!========== (([0], [180]), 30)
Max_pair_as_words ========== (["the"], ["same"])

Iteration number:========== 152
Max pair !!!========== (([16], [50]), 30)
Max_pair_as_words ========== (["you"], ["are"])

Iteration number:========== 153
Max pair !!!========== (([4], [230]), 30)
Max_pair_as_words ========== (["to"], ["hear"])

Iteration number:========== 154
Max pair !!!========== (([23], [209]), 30)
Max_pair_as_words ========== (["He"], ["turned"])

Iteration number:========== 155
Max pair !!!========== (([386], [29]), 29)
Max_pair_as_words ========== (["Must"], ["be"])

Iteration number:========== 156
Max pair !!!========== (([167], [29]), 29)
Max_pair_as_words ========== (["must"], ["be"])

Iteration number:========== 157
Max pair !!!========== (([31], [2]), 29)
Max_pair_as_words ========== (["up"], ["and"])

Iteration number:========== 158
Max pair !!!========== (([8], [90]), 29)
Max_pair_as_words ========== (["I"], ["could"])

Iteration number:========== 159
Max pair !!!========== (([12], [11]), 28)
Max_pair_as_words ========== (["was"], ["it"])

Iteration number:========== 160
Max pair !!!========== (([24], [121]), 28)
Max_pair_as_words ========== (["all"], ["right"])

Iteration number:========== 161
Max pair !!!========== (([1], [45]), 28)
Max_pair_as_words ========== (["of"], ["an"])

Iteration number:========== 162
Max pair !!!========== (([145], [797]), 28)
Max_pair_as_words ========== (["John"], ["Eglinton"])

Iteration number:========== 163
Max pair !!!========== (([35], [107]), 28)
Max_pair_as_words ========== (["they"], ["say"])

Iteration number:========== 164
Max pair !!!========== (([8], [299]), 28)
Max_pair_as_words ========== (["I"], ["suppose"])

Iteration number:========== 165
Max pair !!!========== (([934], [838]), 28)
Max_pair_as_words ========== (["Nosey"], ["Flynn"])

Iteration number:========== 166
Max pair !!!========== (([18], [7]), 28)
Max_pair_as_words ========== (["is"], ["he"])

Iteration number:========== 167
Max pair !!!========== (([145], [818]), 28)
Max_pair_as_words ========== (["John"], ["Wyse"])

Iteration number:========== 168
Max pair !!!========== (([47], [7]), 28)
Max_pair_as_words ========== (["And"], ["he"])

Iteration number:========== 169
Max pair !!!========== (([35], [50]), 28)
Max_pair_as_words ========== (["they"], ["are"])

Iteration number:========== 170
Max pair !!!========== (([19], [3]), 28)
Max_pair_as_words ========== (["at"], ["a"])

Iteration number:========== 171
Max pair !!!========== (([6], [184]), 27)
Max_pair_as_words ========== (["his"], ["own"])

Iteration number:========== 172
Max pair !!!========== (([7], [146]), 27)
Max_pair_as_words ========== (["he"], ["asked"])

Iteration number:========== 173
Max pair !!!========== (([76], [0]), 27)
Max_pair_as_words ========== (["off"], ["the"])

Iteration number:========== 174
Max pair !!!========== (([362], [0]), 27)
Max_pair_as_words ========== (["From"], ["the"])

Iteration number:========== 175
Max pair !!!========== (([567], [16]), 27)
Max_pair_as_words ========== (["Have"], ["you"])

Iteration number:========== 176
Max pair !!!========== (([202], [16]), 27)
Max_pair_as_words ========== (["tell"], ["you"])

Iteration number:========== 177
Max pair !!!========== (([57], [7]), 27)
Max_pair_as_words ========== (["if"], ["he"])

Iteration number:========== 178
Max pair !!!========== (([17], [3]), 27)
Max_pair_as_words ========== (["him"], ["a"])

Iteration number:========== 179
Max pair !!!========== (([142], [4]), 26)
Max_pair_as_words ========== (["going"], ["to"])

Iteration number:========== 180
Max pair !!!========== (([199], [50]), 26)
Max_pair_as_words ========== (["We"], ["are"])

Iteration number:========== 181
Max pair !!!========== (([1], [63]), 26)
Max_pair_as_words ========== (["of"], ["this"])

Iteration number:========== 182
Max pair !!!========== (([10], [17]), 26)
Max_pair_as_words ========== (["with"], ["him"])

Iteration number:========== 183
Max pair !!!========== (([0], [425]), 26)
Max_pair_as_words ========== (["the"], ["professor"])

Iteration number:========== 184
Max pair !!!========== (([9], [7]), 25)
Max_pair_as_words ========== (["that"], ["he"])

Iteration number:========== 185
Max pair !!!========== (([6], [83]), 25)
Max_pair_as_words ========== (["his"], ["eyes"])

Iteration number:========== 186
Max pair !!!========== (([406], [580]), 25)
Max_pair_as_words ========== (["Ben"], ["Dollard"])

Iteration number:========== 187
Max pair !!!========== (([13], [17]), 25)
Max_pair_as_words ========== (["on"], ["him"])

Iteration number:========== 188
Max pair !!!========== (([46], [12]), 25)
Max_pair_as_words ========== (["there"], ["was"])

Iteration number:========== 189
Max pair !!!========== (([26], [15]), 25)
Max_pair_as_words ========== (["from"], ["her"])

Iteration number:========== 190
Max pair !!!========== (([208], [696]), 25)
Max_pair_as_words ========== (["Father"], ["Cowley"])

Iteration number:========== 191
Max pair !!!========== (([55], [43]), 25)
Max_pair_as_words ========== (["says"], ["Bloom"])

Iteration number:========== 192
Max pair !!!========== (([89], [42]), 25)
Max_pair_as_words ========== (["we"], ["have"])

Iteration number:========== 193
Max pair !!!========== (([23], [98]), 25)
Max_pair_as_words ========== (["He"], ["has"])

Iteration number:========== 194
Max pair !!!========== (([222], [0]), 25)
Max_pair_as_words ========== (["against"], ["the"])

Iteration number:========== 195
Max pair !!!========== (([30], [0]), 24)
Max_pair_as_words ========== (["out"], ["the"])

Iteration number:========== 196
Max pair !!!========== (([14], [103]), 24)
Max_pair_as_words ========== (["for"], ["us"])

Iteration number:========== 197
Max pair !!!========== (([765], [767]), 24)
Max_pair_as_words ========== (["Corny"], ["Kelleher"])

Iteration number:========== 198
Max pair !!!========== (([5], [11]), 24)
Max_pair_as_words ========== (["in"], ["it"])

Iteration number:========== 199
Max pair !!!========== (([8], [217]), 24)
Max_pair_as_words ========== (["I"], ["want"])

Iteration number:========== 200
Max pair !!!========== (([28], [4]), 24)
Max_pair_as_words ========== (["me"], ["to"])

Iteration number:========== 201
Max pair !!!========== (([3], [68]), 24)
Max_pair_as_words ========== (["a"], ["man"])

Iteration number:========== 202
Max pair !!!========== (([8], [306]), 24)
Max_pair_as_words ========== (["I"], ["feel"])

Iteration number:========== 203
Max pair !!!========== (([2], [1]), 24)
Max_pair_as_words ========== (["and"], ["of"])

Iteration number:========== 204
Max pair !!!========== (([1], [108]), 24)
Max_pair_as_words ========== (["of"], ["our"])

Iteration number:========== 205
Max pair !!!========== (([0], [59]), 24)
Max_pair_as_words ========== (["the"], ["old"])

Iteration number:========== 206
Max pair !!!========== (([3], [99]), 24)
Max_pair_as_words ========== (["a"], ["little"])

Iteration number:========== 207
Max pair !!!========== (([25], [11]), 24)
Max_pair_as_words ========== (["as"], ["it"])

Iteration number:========== 208
Max pair !!!========== (([8], [34]), 24)
Max_pair_as_words ========== (["I"], ["had"])

Iteration number:========== 209
Max pair !!!========== (([91], [157]), 23)
Max_pair_as_words ========== (["more"], ["than"])

Iteration number:========== 210
Max pair !!!========== (([72], [12]), 23)
Max_pair_as_words ========== (["who"], ["was"])

Iteration number:========== 211
Max pair !!!========== (([19], [24]), 23)
Max_pair_as_words ========== (["at"], ["all"])

Iteration number:========== 212
Max pair !!!========== (([10], [45]), 23)
Max_pair_as_words ========== (["with"], ["an"])

Iteration number:========== 213
Max pair !!!========== (([287], [1]), 23)
Max_pair_as_words ========== (["kind"], ["of"])

Iteration number:========== 214
Max pair !!!========== (([4], [131]), 23)
Max_pair_as_words ========== (["to"], ["go"])

Iteration number:========== 215
Max pair !!!========== (([467], [173]), 23)
Max_pair_as_words ========== (["Of"], ["course"])

Iteration number:========== 216
Max pair !!!========== (([17], [5]), 22)
Max_pair_as_words ========== (["him"], ["in"])

Iteration number:========== 217
Max pair !!!========== (([5], [24]), 22)
Max_pair_as_words ========== (["in"], ["all"])

Iteration number:========== 218
Max pair !!!========== (([339], [9]), 22)
Max_pair_as_words ========== (["Is"], ["that"])

Iteration number:========== 219
Max pair !!!========== (([1138], [16]), 22)
Max_pair_as_words ========== (["Thank"], ["you"])

Iteration number:========== 220
Max pair !!!========== (([11], [4]), 22)
Max_pair_as_words ========== (["it"], ["to"])

Iteration number:========== 221
Max pair !!!========== (([61], [6]), 22)
Max_pair_as_words ========== (["over"], ["his"])

Iteration number:========== 222
Max pair !!!========== (([2], [24]), 22)
Max_pair_as_words ========== (["and"], ["all"])

Iteration number:========== 223
Max pair !!!========== (([0], [114]), 22)
Max_pair_as_words ========== (["the"], ["first"])

Iteration number:========== 224
Max pair !!!========== (([1], [93]), 22)
Max_pair_as_words ========== (["of"], ["those"])

Iteration number:========== 225
Max pair !!!========== (([0], [200]), 22)
Max_pair_as_words ========== (["the"], ["door"])

Iteration number:========== 226
Max pair !!!========== (([79], [43]), 22)
Max_pair_as_words ========== (["did"], ["Bloom"])

Iteration number:========== 227
Max pair !!!========== (([5], [56]), 21)
Max_pair_as_words ========== (["in"], ["your"])

Iteration number:========== 228
Max pair !!!========== (([452], [3]), 21)
Max_pair_as_words ========== (["There’s"], ["a"])

Iteration number:========== 229
Max pair !!!========== (([101], [3]), 21)
Max_pair_as_words ========== (["In"], ["a"])

Iteration number:========== 230
Max pair !!!========== (([301], [13]), 21)
Max_pair_as_words ========== (["Come"], ["on"])

Iteration number:========== 231
Max pair !!!========== (([5], [45]), 21)
Max_pair_as_words ========== (["in"], ["an"])

Iteration number:========== 232
Max pair !!!========== (([293], [22]), 21)
Max_pair_as_words ========== (["Lenehan"], ["said"])

Iteration number:========== 233
Max pair !!!========== (([1278], [726]), 21)
Max_pair_as_words ========== (["FIRST"], ["WATCH"])

Iteration number:========== 234
Max pair !!!========== (([19], [15]), 21)
Max_pair_as_words ========== (["at"], ["her"])

Iteration number:========== 235
Max pair !!!========== (([51], [7]), 21)
Max_pair_as_words ========== (["which"], ["he"])

Iteration number:========== 236
Max pair !!!========== (([130], [0]), 21)
Max_pair_as_words ========== (["before"], ["the"])

Iteration number:========== 237
Max pair !!!========== (([23], [34]), 21)
Max_pair_as_words ========== (["He"], ["had"])

Iteration number:========== 238
Max pair !!!========== (([14], [28]), 21)
Max_pair_as_words ========== (["for"], ["me"])

Iteration number:========== 239
Max pair !!!========== (([1], [51]), 21)
Max_pair_as_words ========== (["of"], ["which"])

Iteration number:========== 240
Max pair !!!========== (([4], [44]), 21)
Max_pair_as_words ========== (["to"], ["one"])

Iteration number:========== 241
Max pair !!!========== (([2], [62]), 21)
Max_pair_as_words ========== (["and"], ["then"])

Iteration number:========== 242
Max pair !!!========== (([4], [169]), 21)
Max_pair_as_words ========== (["to"], ["make"])

Iteration number:========== 243
Max pair !!!========== (([8], [167]), 21)
Max_pair_as_words ========== (["I"], ["must"])

Iteration number:========== 244
Max pair !!!========== (([1], [37]), 21)
Max_pair_as_words ========== (["of"], ["my"])

Iteration number:========== 245
Max pair !!!========== (([25], [35]), 21)
Max_pair_as_words ========== (["as"], ["they"])

Iteration number:========== 246
Max pair !!!========== (([2], [8]), 21)
Max_pair_as_words ========== (["and"], ["I"])

Iteration number:========== 247
Max pair !!!========== (([427], [28]), 21)
Max_pair_as_words ========== (["Let"], ["me"])

Iteration number:========== 248
Max pair !!!========== (([181], [7]), 20)
Max_pair_as_words ========== (["Then"], ["he"])

Iteration number:========== 249
Max pair !!!========== (([94], [42]), 20)
Max_pair_as_words ========== (["You"], ["have"])

Iteration number:========== 250
Max pair !!!========== (([7], [98]), 20)
Max_pair_as_words ========== (["he"], ["has"])

Iteration number:========== 251
Max pair !!!========== (([2], [46]), 20)
Max_pair_as_words ========== (["and"], ["there"])

Iteration number:========== 252
Max pair !!!========== (([216], [0]), 20)
Max_pair_as_words ========== (["upon"], ["the"])

Iteration number:========== 253
Max pair !!!========== (([23], [268]), 20)
Max_pair_as_words ========== (["He"], ["looked"])

Iteration number:========== 254
Max pair !!!========== (([0], [478]), 20)
Max_pair_as_words ========== (["the"], ["whole"])

Iteration number:========== 255
Max pair !!!========== (([841], [1340]), 20)
Max_pair_as_words ========== (["PRIVATE"], ["CARR"])

Iteration number:========== 256
Max pair !!!========== (([1], [17]), 20)
Max_pair_as_words ========== (["of"], ["him"])

Iteration number:========== 257
Max pair !!!========== (([33], [3]), 20)
Max_pair_as_words ========== (["not"], ["a"])

Iteration number:========== 258
Max pair !!!========== (([13], [11]), 20)
Max_pair_as_words ========== (["on"], ["it"])

Iteration number:========== 259
Max pair !!!========== (([4], [2]), 20)
Max_pair_as_words ========== (["to"], ["and"])

Iteration number:========== 260
Max pair !!!========== (([84], [16]), 20)
Max_pair_as_words ========== (["will"], ["you"])

Iteration number:========== 261
Max pair !!!========== (([0], [163]), 20)
Max_pair_as_words ========== (["the"], ["last"])

Iteration number:========== 262
Max pair !!!========== (([5], [88]), 19)
Max_pair_as_words ========== (["in"], ["its"])

Iteration number:========== 263
Max pair !!!========== (([12], [9]), 19)
Max_pair_as_words ========== (["was"], ["that"])

Iteration number:========== 264
Max pair !!!========== (([6], [127]), 19)
Max_pair_as_words ========== (["his"], ["name"])

Iteration number:========== 265
Max pair !!!========== (([27], [3]), 19)
Max_pair_as_words ========== (["or"], ["a"])

Iteration number:========== 266
Max pair !!!========== (([389], [16]), 19)
Max_pair_as_words ========== (["Did"], ["you"])

Iteration number:========== 267
Max pair !!!========== (([723], [1403]), 19)
Max_pair_as_words ========== (["MRS"], ["BREEN"])

Iteration number:========== 268
Max pair !!!========== (([21], [6]), 19)
Max_pair_as_words ========== (["by"], ["his"])

Iteration number:========== 269
Max pair !!!========== (([7], [75]), 19)
Max_pair_as_words ========== (["he"], ["would"])

Iteration number:========== 270
Max pair !!!========== (([8], [107]), 19)
Max_pair_as_words ========== (["I"], ["say"])

Iteration number:========== 271
Max pair !!!========== (([252], [0]), 19)
Max_pair_as_words ========== (["along"], ["the"])

Iteration number:========== 272
Max pair !!!========== (([13], [39]), 19)
Max_pair_as_words ========== (["on"], ["their"])

Iteration number:========== 273
Max pair !!!========== (([196], [0]), 19)
Max_pair_as_words ========== (["took"], ["the"])

Iteration number:========== 274
Max pair !!!========== (([31], [3]), 19)
Max_pair_as_words ========== (["up"], ["a"])

Iteration number:========== 275
Max pair !!!========== (([538], [4]), 19)
Max_pair_as_words ========== (["began"], ["to"])

Iteration number:========== 276
Max pair !!!========== (([328], [0]), 19)
Max_pair_as_words ========== (["By"], ["the"])

Iteration number:========== 277
Max pair !!!========== (([14], [6]), 19)
Max_pair_as_words ========== (["for"], ["his"])

Iteration number:========== 278
Max pair !!!========== (([94], [86]), 19)
Max_pair_as_words ========== (["You"], ["know"])

Iteration number:========== 279
Max pair !!!========== (([763], [489]), 18)
Max_pair_as_words ========== (["Blazes"], ["Boylan"])

Iteration number:========== 280
Max pair !!!========== (([102], [0]), 18)
Max_pair_as_words ========== (["But"], ["the"])

Iteration number:========== 281
Max pair !!!========== (([4], [551]), 18)
Max_pair_as_words ========== (["to"], ["speak"])

Iteration number:========== 282
Max pair !!!========== (([94], [50]), 18)
Max_pair_as_words ========== (["You"], ["are"])

Iteration number:========== 283
Max pair !!!========== (([55], [133]), 18)
Max_pair_as_words ========== (["says"], ["J"])

Iteration number:========== 284
Max pair !!!========== (([167], [42]), 18)
Max_pair_as_words ========== (["must"], ["have"])

Iteration number:========== 285
Max pair !!!========== (([460], [0]), 18)
Max_pair_as_words ========== (["across"], ["the"])

Iteration number:========== 286
Max pair !!!========== (([71], [0]), 18)
Max_pair_as_words ========== (["after"], ["the"])

Iteration number:========== 287
Max pair !!!========== (([54], [16]), 18)
Max_pair_as_words ========== (["what"], ["you"])

Iteration number:========== 288
Max pair !!!========== (([432], [22]), 18)
Max_pair_as_words ========== (["Haines"], ["said"])

Iteration number:========== 289
Max pair !!!========== (([135], [18]), 18)
Max_pair_as_words ========== (["That"], ["is"])

Iteration number:========== 290
Max pair !!!========== (([1], [147]), 18)
Max_pair_as_words ========== (["of"], ["life"])

Iteration number:========== 291
Max pair !!!========== (([4], [16]), 18)
Max_pair_as_words ========== (["to"], ["you"])

Iteration number:========== 292
Max pair !!!========== (([553], [103]), 18)
Max_pair_as_words ========== (["Give"], ["us"])

Iteration number:========== 293
Max pair !!!========== (([0], [1144]), 18)
Max_pair_as_words ========== (["the"], ["editor"])

Iteration number:========== 294
Max pair !!!========== (([36], [796]), 17)
Max_pair_as_words ========== (["Mr"], ["Best"])

Iteration number:========== 295
Max pair !!!========== (([13], [37]), 17)
Max_pair_as_words ========== (["on"], ["my"])

Iteration number:========== 296
Max pair !!!========== (([82], [34]), 17)
Max_pair_as_words ========== (["She"], ["had"])

Iteration number:========== 297
Max pair !!!========== (([100], [83]), 17)
Max_pair_as_words ========== (["His"], ["eyes"])

Iteration number:========== 298
Max pair !!!========== (([42], [16]), 17)
Max_pair_as_words ========== (["have"], ["you"])

Iteration number:========== 299
Max pair !!!========== (([0], [187]), 17)
Max_pair_as_words ========== (["the"], ["world"])

Iteration number:========== 300
Max pair !!!========== (([1], [248]), 17)
Max_pair_as_words ========== (["of"], ["Dublin"])

Iteration number:========== 301
Max pair !!!========== (([10], [16]), 17)
Max_pair_as_words ========== (["with"], ["you"])

Iteration number:========== 302
Max pair !!!========== (([53], [7]), 17)
Max_pair_as_words ========== (["when"], ["he"])

Iteration number:========== 303
Max pair !!!========== (([145], [354]), 17)
Max_pair_as_words ========== (["John"], ["Henry"])

Iteration number:========== 304
Max pair !!!========== (([79], [16]), 17)
Max_pair_as_words ========== (["did"], ["you"])

Iteration number:========== 305
Max pair !!!========== (([8], [128]), 17)
Max_pair_as_words ========== (["I"], ["never"])

Iteration number:========== 306
Max pair !!!========== (([199], [42]), 17)
Max_pair_as_words ========== (["We"], ["have"])

Iteration number:========== 307
Max pair !!!========== (([480], [0]), 17)
Max_pair_as_words ========== (["among"], ["the"])

Iteration number:========== 308
Max pair !!!========== (([1036], [14, 103]), 17)
Max_pair_as_words ========== (["pray"], ["for", "us"])

Iteration number:========== 309
Max pair !!!========== (([47], [35]), 17)
Max_pair_as_words ========== (["And"], ["they"])

Iteration number:========== 310
Max pair !!!========== (([106], [50]), 17)
Max_pair_as_words ========== (["They"], ["are"])

Iteration number:========== 311
Max pair !!!========== (([8], [533]), 17)
Max_pair_as_words ========== (["I"], ["hope"])

Iteration number:========== 312
Max pair !!!========== (([545], [697]), 17)
Max_pair_as_words ========== (["miss"], ["Douce"])

Iteration number:========== 313
Max pair !!!========== (([8], [387]), 17)
Max_pair_as_words ========== (["I"], ["believe"])

Iteration number:========== 314
Max pair !!!========== (([13], [16]), 17)
Max_pair_as_words ========== (["on"], ["you"])

Iteration number:========== 315
Max pair !!!========== (([191], [28]), 17)
Max_pair_as_words ========== (["told"], ["me"])

Iteration number:========== 316
Max pair !!!========== (([11], [2]), 17)
Max_pair_as_words ========== (["it"], ["and"])

Iteration number:========== 317
Max pair !!!========== (([17], [4]), 17)
Max_pair_as_words ========== (["him"], ["to"])

Iteration number:========== 318
Max pair !!!========== (([3], [118]), 17)
Max_pair_as_words ========== (["a"], ["long"])

Iteration number:========== 319
Max pair !!!========== (([135], [12]), 17)
Max_pair_as_words ========== (["That"], ["was"])

Iteration number:========== 320
Max pair !!!========== (([205], [16, 86]), 17)
Max_pair_as_words ========== (["don’t"], ["you", "know"])

Iteration number:========== 321
Max pair !!!========== (([349], [4]), 16)
Max_pair_as_words ========== (["used"], ["to"])

Iteration number:========== 322
Max pair !!!========== (([149], [0]), 16)
Max_pair_as_words ========== (["All"], ["the"])

Iteration number:========== 323
Max pair !!!========== (([215], [8]), 16)
Max_pair_as_words ========== (["till"], ["I"])

Iteration number:========== 324
Max pair !!!========== (([36], [748]), 16)
Max_pair_as_words ========== (["Mr"], ["Kernan"])

Iteration number:========== 325
Max pair !!!========== (([5], [51]), 16)
Max_pair_as_words ========== (["in"], ["which"])

Iteration number:========== 326
Max pair !!!========== (([1003], [1277]), 16)
Max_pair_as_words ========== (["Davy"], ["Byrne"])

Iteration number:========== 327
Max pair !!!========== (([89], [50]), 16)
Max_pair_as_words ========== (["we"], ["are"])

Iteration number:========== 328
Max pair !!!========== (([10], [39]), 16)
Max_pair_as_words ========== (["with"], ["their"])

Iteration number:========== 329
Max pair !!!========== (([47], [62]), 16)
Max_pair_as_words ========== (["And"], ["then"])

Iteration number:========== 330
Max pair !!!========== (([515], [4]), 16)
Max_pair_as_words ========== (["ought"], ["to"])

Iteration number:========== 331
Max pair !!!========== (([161], [0]), 16)
Max_pair_as_words ========== (["To"], ["the"])

Iteration number:========== 332
Max pair !!!========== (([385], [0]), 16)
Max_pair_as_words ========== (["At"], ["the"])

Iteration number:========== 333
Max pair !!!========== (([11], [30]), 16)
Max_pair_as_words ========== (["it"], ["out"])

Iteration number:========== 334
Max pair !!!========== (([339], [11]), 16)
Max_pair_as_words ========== (["Is"], ["it"])

Iteration number:========== 335
Max pair !!!========== (([31], [4, 0]), 16)
Max_pair_as_words ========== (["up"], ["to", "the"])

Iteration number:========== 336
Max pair !!!========== (([599], [19]), 16)
Max_pair_as_words ========== (["Look"], ["at"])

Iteration number:========== 337
Max pair !!!========== (([313], [16, 86]), 16)
Max_pair_as_words ========== (["Do"], ["you", "know"])

Iteration number:========== 338
Max pair !!!========== (([31], [4]), 16)
Max_pair_as_words ========== (["up"], ["to"])

Iteration number:========== 339
Max pair !!!========== (([21], [45]), 16)
Max_pair_as_words ========== (["by"], ["an"])

Iteration number:========== 340
Max pair !!!========== (([98], [134]), 16)
Max_pair_as_words ========== (["has"], ["been"])

Iteration number:========== 341
Max pair !!!========== (([23], [150]), 16)
Max_pair_as_words ========== (["He"], ["went"])

Iteration number:========== 342
Max pair !!!========== (([2], [49]), 16)
Max_pair_as_words ========== (["and"], ["so"])

Iteration number:========== 343
Max pair !!!========== (([5], [2]), 16)
Max_pair_as_words ========== (["in"], ["and"])

Iteration number:========== 344
Max pair !!!========== (([27], [0]), 16)
Max_pair_as_words ========== (["or"], ["the"])

Iteration number:========== 345
Max pair !!!========== (([290], [0]), 16)
Max_pair_as_words ========== (["That’s"], ["the"])

Iteration number:========== 346
Max pair !!!========== (([2], [32]), 16)
Max_pair_as_words ========== (["and"], ["she"])

Iteration number:========== 347
Max pair !!!========== (([3], [358]), 16)
Max_pair_as_words ========== (["a"], ["few"])

Iteration number:========== 348
Max pair !!!========== (([14], [24]), 16)
Max_pair_as_words ========== (["for"], ["all"])

Iteration number:========== 349
Max pair !!!========== (([16], [42]), 16)
Max_pair_as_words ========== (["you"], ["have"])

Iteration number:========== 350
Max pair !!!========== (([81], [3]), 15)
Max_pair_as_words ========== (["into"], ["a"])

Iteration number:========== 351
Max pair !!!========== (([23], [350]), 15)
Max_pair_as_words ========== (["He"], ["walked"])

Iteration number:========== 352
Max pair !!!========== (([0], [139]), 15)
Max_pair_as_words ========== (["the"], ["very"])

Iteration number:========== 353
Max pair !!!========== (([662], [22]), 15)
Max_pair_as_words ========== (["O’Molloy"], ["said"])

Iteration number:========== 354
Max pair !!!========== (([117], [1740]), 15)
Max_pair_as_words ========== (["THE"], ["NYMPH"])

Iteration number:========== 355
Max pair !!!========== (([1413], [1051]), 15)
Max_pair_as_words ========== (["O’Madden"], ["Burke"])

Iteration number:========== 356
Max pair !!!========== (([2], [1, 0]), 15)
Max_pair_as_words ========== (["and"], ["of", "the"])

Iteration number:========== 357
Max pair !!!========== (([3], [324]), 15)
Max_pair_as_words ========== (["a"], ["moment"])

Iteration number:========== 358
Max pair !!!========== (([22], [7]), 15)
Max_pair_as_words ========== (["said"], ["he"])

Iteration number:========== 359
Max pair !!!========== (([1745], [726]), 15)
Max_pair_as_words ========== (["SECOND"], ["WATCH"])

Iteration number:========== 360
Max pair !!!========== (([77], [2]), 15)
Max_pair_as_words ========== (["back"], ["and"])

Iteration number:========== 361
Max pair !!!========== (([6], [162]), 15)
Max_pair_as_words ========== (["his"], ["hat"])

Iteration number:========== 362
Max pair !!!========== (([0], [1075]), 15)
Max_pair_as_words ========== (["the"], ["rev"])

Iteration number:========== 363
Max pair !!!========== (([41], [31]), 15)
Max_pair_as_words ========== (["them"], ["up"])

Iteration number:========== 364
Max pair !!!========== (([361], [29]), 15)
Max_pair_as_words ========== (["may"], ["be"])

Iteration number:========== 365
Max pair !!!========== (([82], [12]), 15)
Max_pair_as_words ========== (["She"], ["was"])

Iteration number:========== 366
Max pair !!!========== (([322], [697]), 15)
Max_pair_as_words ========== (["Miss"], ["Douce"])

Iteration number:========== 367
Max pair !!!========== (([308], [3]), 15)
Max_pair_as_words ========== (["He’s"], ["a"])

Iteration number:========== 368
Max pair !!!========== (([8], [422]), 15)
Max_pair_as_words ========== (["I"], ["didn’t"])

Iteration number:========== 369
Max pair !!!========== (([67], [18, 11]), 15)
Max_pair_as_words ========== (["What"], ["is", "it"])

Iteration number:========== 370
Max pair !!!========== (([30], [3]), 15)
Max_pair_as_words ========== (["out"], ["a"])

Iteration number:========== 371
Max pair !!!========== (([8], [473]), 15)
Max_pair_as_words ========== (["I"], ["remember"])

Iteration number:========== 372
Max pair !!!========== (([16], [52]), 15)
Max_pair_as_words ========== (["you"], ["were"])

Iteration number:========== 373
Max pair !!!========== (([36], [1413, 1051]), 14)
Max_pair_as_words ========== (["Mr"], ["O’Madden", "Burke"])

Iteration number:========== 374
Max pair !!!========== (([517], [837]), 14)
Max_pair_as_words ========== (["Cissy"], ["Caffrey"])

Iteration number:========== 375
Max pair !!!========== (([55], [402]), 14)
Max_pair_as_words ========== (["says"], ["Ned"])

Iteration number:========== 376
Max pair !!!========== (([2], [39]), 14)
Max_pair_as_words ========== (["and"], ["their"])

Iteration number:========== 377
Max pair !!!========== (([35], [42]), 14)
Max_pair_as_words ========== (["they"], ["have"])

Iteration number:========== 378
Max pair !!!========== (([326], [933]), 14)
Max_pair_as_words ========== (["Project"], ["Gutenberg"])

Iteration number:========== 379
Max pair !!!========== (([0], [311]), 14)
Max_pair_as_words ========== (["the"], ["best"])

Iteration number:========== 380
Max pair !!!========== (([17], [0]), 14)
Max_pair_as_words ========== (["him"], ["the"])

Iteration number:========== 381
Max pair !!!========== (([67], [18]), 14)
Max_pair_as_words ========== (["What"], ["is"])

Iteration number:========== 382
Max pair !!!========== (([51], [12]), 14)
Max_pair_as_words ========== (["which"], ["was"])

Iteration number:========== 383
Max pair !!!========== (([8], [228]), 14)
Max_pair_as_words ========== (["I"], ["heard"])

Iteration number:========== 384
Max pair !!!========== (([6], [112]), 14)
Max_pair_as_words ========== (["his"], ["face"])

Iteration number:========== 385
Max pair !!!========== (([425], [1345]), 14)
Max_pair_as_words ========== (["professor"], ["MacHugh"])

Iteration number:========== 386
Max pair !!!========== (([23], [312]), 14)
Max_pair_as_words ========== (["He"], ["stood"])

Iteration number:========== 387
Max pair !!!========== (([64], [414]), 14)
Max_pair_as_words ========== (["Stephen"], ["answered"])

Iteration number:========== 388
Max pair !!!========== (([1], [601]), 14)
Max_pair_as_words ========== (["of"], ["bread"])

Iteration number:========== 389
Max pair !!!========== (([4], [64]), 14)
Max_pair_as_words ========== (["to"], ["Stephen"])

Iteration number:========== 390
Max pair !!!========== (([9], [97]), 14)
Max_pair_as_words ========== (["that"], ["way"])

Iteration number:========== 391
Max pair !!!========== (([50], [0]), 14)
Max_pair_as_words ========== (["are"], ["the"])

Iteration number:========== 392
Max pair !!!========== (([19], [250]), 14)
Max_pair_as_words ========== (["at"], ["once"])

Iteration number:========== 393
Max pair !!!========== (([202], [28]), 14)
Max_pair_as_words ========== (["tell"], ["me"])

Iteration number:========== 394
Max pair !!!========== (([4], [74]), 14)
Max_pair_as_words ========== (["to"], ["do"])

Iteration number:========== 395
Max pair !!!========== (([764], [1408]), 14)
Max_pair_as_words ========== (["Bob"], ["Doran"])

Iteration number:========== 396
Max pair !!!========== (([220], [0]), 14)
Max_pair_as_words ========== (["between"], ["the"])

Iteration number:========== 397
Max pair !!!========== (([55], [145, 818]), 14)
Max_pair_as_words ========== (["says"], ["John", "Wyse"])

Iteration number:========== 398
Max pair !!!========== (([1129], [16]), 13)
Max_pair_as_words ========== (["Can"], ["you"])

Iteration number:========== 399
Max pair !!!========== (([183], [29]), 13)
Max_pair_as_words ========== (["might"], ["be"])

Iteration number:========== 400
Max pair !!!========== (([0], [123]), 13)
Max_pair_as_words ========== (["the"], ["night"])

Iteration number:========== 401
Max pair !!!========== (([6], [186]), 13)
Max_pair_as_words ========== (["his"], ["hands"])

Iteration number:========== 402
Max pair !!!========== (([2], [65]), 13)
Max_pair_as_words ========== (["and"], ["no"])

Iteration number:========== 403
Max pair !!!========== (([536], [159]), 13)
Max_pair_as_words ========== (["Simon"], ["Dedalus"])

Iteration number:========== 404
Max pair !!!========== (([329], [3]), 13)
Max_pair_as_words ========== (["It’s"], ["a"])

Iteration number:========== 405
Max pair !!!========== (([18], [33]), 13)
Max_pair_as_words ========== (["is"], ["not"])

Iteration number:========== 406
Max pair !!!========== (([841], [1969]), 13)
Max_pair_as_words ========== (["PRIVATE"], ["COMPTON"])

Iteration number:========== 407
Max pair !!!========== (([15], [83]), 13)
Max_pair_as_words ========== (["her"], ["eyes"])

Iteration number:========== 408
Max pair !!!========== (([8], [74]), 13)
Max_pair_as_words ========== (["I"], ["do"])

Iteration number:========== 409
Max pair !!!========== (([2], [10]), 13)
Max_pair_as_words ========== (["and"], ["with"])

Iteration number:========== 410
Max pair !!!========== (([45], [59]), 13)
Max_pair_as_words ========== (["an"], ["old"])

Iteration number:========== 411
Max pair !!!========== (([624], [1743]), 13)
Max_pair_as_words ========== (["Tom"], ["Rochford"])

Iteration number:========== 412
Max pair !!!========== (([125], [3]), 13)
Max_pair_as_words ========== (["under"], ["a"])

Iteration number:========== 413
Max pair !!!========== (([5], [275]), 13)
Max_pair_as_words ========== (["in"], ["bed"])

Iteration number:========== 414
Max pair !!!========== (([98], [0]), 13)
Max_pair_as_words ========== (["has"], ["the"])

Iteration number:========== 415
Max pair !!!========== (([766], [1128]), 13)
Max_pair_as_words ========== (["Edy"], ["Boardman"])

Iteration number:========== 416
Max pair !!!========== (([41], [24]), 13)
Max_pair_as_words ========== (["them"], ["all"])

Iteration number:========== 417
Max pair !!!========== (([339], [7]), 13)
Max_pair_as_words ========== (["Is"], ["he"])

Iteration number:========== 418
Max pair !!!========== (([15], [87]), 13)
Max_pair_as_words ========== (["her"], ["hand"])

Iteration number:========== 419
Max pair !!!========== (([10], [28]), 13)
Max_pair_as_words ========== (["with"], ["me"])

Iteration number:========== 420
Max pair !!!========== (([0], [68]), 13)
Max_pair_as_words ========== (["the"], ["man"])

Iteration number:========== 421
Max pair !!!========== (([172], [1]), 13)
Max_pair_as_words ========== (["voice"], ["of"])

Iteration number:========== 422
Max pair !!!========== (([48], [9]), 13)
Max_pair_as_words ========== (["about"], ["that"])

Iteration number:========== 423
Max pair !!!========== (([55], [36]), 13)
Max_pair_as_words ========== (["says"], ["Mr"])

Iteration number:========== 424
Max pair !!!========== (([152], [3]), 13)
Max_pair_as_words ========== (["made"], ["a"])

Iteration number:========== 425
Max pair !!!========== (([106], [52]), 13)
Max_pair_as_words ========== (["They"], ["were"])

Iteration number:========== 426
Max pair !!!========== (([8], [230]), 13)
Max_pair_as_words ========== (["I"], ["hear"])

Iteration number:========== 427
Max pair !!!========== (([236], [3]), 13)
Max_pair_as_words ========== (["take"], ["a"])

Iteration number:========== 428
Max pair !!!========== (([81], [6]), 13)
Max_pair_as_words ========== (["into"], ["his"])

Iteration number:========== 429
Max pair !!!========== (([246], [7, 18]), 13)
Max_pair_as_words ========== (["There"], ["he", "is"])

Iteration number:========== 430
Max pair !!!========== (([338], [29]), 13)
Max_pair_as_words ========== (["should"], ["be"])

Iteration number:========== 431
Max pair !!!========== (([116], [12, 3]), 13)
Max_pair_as_words ========== (["It"], ["was", "a"])

Iteration number:========== 432
Max pair !!!========== (([8], [367]), 13)
Max_pair_as_words ========== (["I"], ["seen"])

Iteration number:========== 433
Max pair !!!========== (([21], [136]), 13)
Max_pair_as_words ========== (["by"], ["God"])

Iteration number:========== 434
Max pair !!!========== (([14], [1145]), 12)
Max_pair_as_words ========== (["for"], ["example"])

Iteration number:========== 435
Max pair !!!========== (([94], [144]), 12)
Max_pair_as_words ========== (["You"], ["can"])

Iteration number:========== 436
Max pair !!!========== (([86], [54]), 12)
Max_pair_as_words ========== (["know"], ["what"])

Iteration number:========== 437
Max pair !!!========== (([36], [420]), 12)
Max_pair_as_words ========== (["Mr"], ["Bloom’s"])

Iteration number:========== 438
Max pair !!!========== (([160], [1173]), 12)
Max_pair_as_words ========== (["Mrs"], ["Breen"])

Iteration number:========== 439
Max pair !!!========== (([799], [22]), 12)
Max_pair_as_words ========== (["M’Coy"], ["said"])

Iteration number:========== 440
Max pair !!!========== (([3], [158]), 12)
Max_pair_as_words ========== (["a"], ["woman"])

Iteration number:========== 441
Max pair !!!========== (([14], [56]), 12)
Max_pair_as_words ========== (["for"], ["your"])

Iteration number:========== 442
Max pair !!!========== (([164], [0]), 12)
Max_pair_as_words ========== (["left"], ["the"])

Iteration number:========== 443
Max pair !!!========== (([79], [0]), 12)
Max_pair_as_words ========== (["did"], ["the"])

Iteration number:========== 444
Max pair !!!========== (([4], [151]), 12)
Max_pair_as_words ========== (["to"], ["come"])

Iteration number:========== 445
Max pair !!!========== (([260], [134]), 12)
Max_pair_as_words ========== (["having"], ["been"])

Iteration number:========== 446
Max pair !!!========== (([316], [85]), 12)
Max_pair_as_words ========== (["each"], ["other"])

Iteration number:========== 447
Max pair !!!========== (([127], [18]), 12)
Max_pair_as_words ========== (["name"], ["is"])

Iteration number:========== 448
Max pair !!!========== (([102], [7]), 12)
Max_pair_as_words ========== (["But"], ["he"])

Iteration number:========== 449
Max pair !!!========== (([9], [7, 12]), 12)
Max_pair_as_words ========== (["that"], ["he", "was"])

Iteration number:========== 450
Max pair !!!========== (([0], [786]), 12)
Max_pair_as_words ========== (["the"], ["sailor"])

Iteration number:========== 451
Max pair !!!========== (([55], [269]), 12)
Max_pair_as_words ========== (["says"], ["Martin"])

Iteration number:========== 452
Max pair !!!========== (([90], [33]), 12)
Max_pair_as_words ========== (["could"], ["not"])

Iteration number:========== 453
Max pair !!!========== (([4], [136]), 12)
Max_pair_as_words ========== (["to"], ["God"])

Iteration number:========== 454
Max pair !!!========== (([61], [15]), 12)
Max_pair_as_words ========== (["over"], ["her"])

Iteration number:========== 455
Max pair !!!========== (([30], [1, 6]), 12)
Max_pair_as_words ========== (["out"], ["of", "his"])

Iteration number:========== 456
Max pair !!!========== (([79], [8]), 12)
Max_pair_as_words ========== (["did"], ["I"])

Iteration number:========== 457
Max pair !!!========== (([8], [233]), 12)
Max_pair_as_words ========== (["I"], ["gave"])

Iteration number:========== 458
Max pair !!!========== (([1196], [1196]), 12)
Max_pair_as_words ========== (["hee"], ["hee"])

Iteration number:========== 459
Max pair !!!========== (([63], [249]), 12)
Max_pair_as_words ========== (["this"], ["morning"])

Iteration number:========== 460
Max pair !!!========== (([26], [1759]), 12)
Max_pair_as_words ========== (["from"], ["afar"])

Iteration number:========== 461
Max pair !!!========== (([14], [39]), 12)
Max_pair_as_words ========== (["for"], ["their"])

Iteration number:========== 462
Max pair !!!========== (([16], [144]), 12)
Max_pair_as_words ========== (["you"], ["can"])

Iteration number:========== 463
Max pair !!!========== (([246], [12, 3]), 12)
Max_pair_as_words ========== (["There"], ["was", "a"])

Iteration number:========== 464
Max pair !!!========== (([2112], [2128]), 12)
Max_pair_as_words ========== (["CORNY"], ["KELLEHER"])

Iteration number:========== 465
Max pair !!!========== (([531], [9]), 12)
Max_pair_as_words ========== (["What’s"], ["that"])

Iteration number:========== 466
Max pair !!!========== (([3], [255]), 12)
Max_pair_as_words ========== (["a"], ["word"])

Iteration number:========== 467
Max pair !!!========== (([13], [51]), 12)
Max_pair_as_words ========== (["on"], ["which"])

Iteration number:========== 468
Max pair !!!========== (([377], [43]), 12)
Max_pair_as_words ========== (["Leopold"], ["Bloom"])

Iteration number:========== 469
Max pair !!!========== (([231], [0]), 12)
Max_pair_as_words ========== (["For"], ["the"])

Iteration number:========== 470
Max pair !!!========== (([24], [6]), 12)
Max_pair_as_words ========== (["all"], ["his"])

Iteration number:========== 471
Max pair !!!========== (([157], [0]), 12)
Max_pair_as_words ========== (["than"], ["the"])

Iteration number:========== 472
Max pair !!!========== (([8], [177]), 12)
Max_pair_as_words ========== (["I"], ["got"])

Iteration number:========== 473
Max pair !!!========== (([94], [52]), 12)
Max_pair_as_words ========== (["You"], ["were"])

Iteration number:========== 474
Max pair !!!========== (([545], [888]), 12)
Max_pair_as_words ========== (["miss"], ["Kennedy"])

Iteration number:========== 475
Max pair !!!========== (([1721], [1]), 12)
Max_pair_as_words ========== (["tribe"], ["of"])

Iteration number:========== 476
Max pair !!!========== (([38], [68]), 12)
Max_pair_as_words ========== (["A"], ["man"])

Iteration number:========== 477
Max pair !!!========== (([26], [39]), 11)
Max_pair_as_words ========== (["from"], ["their"])

Iteration number:========== 478
Max pair !!!========== (([13], [4]), 11)
Max_pair_as_words ========== (["on"], ["to"])

Iteration number:========== 479
Max pair !!!========== (([47], [46]), 11)
Max_pair_as_words ========== (["And"], ["there"])

Iteration number:========== 480
Max pair !!!========== (([0], [393]), 11)
Max_pair_as_words ========== (["the"], ["lord"])

Iteration number:========== 481
Max pair !!!========== (([32], [18]), 11)
Max_pair_as_words ========== (["she"], ["is"])

Iteration number:========== 482
Max pair !!!========== (([8], [22]), 11)
Max_pair_as_words ========== (["I"], ["said"])

Iteration number:========== 483
Max pair !!!========== (([14], [11]), 11)
Max_pair_as_words ========== (["for"], ["it"])

Iteration number:========== 484
Max pair !!!========== (([13], [9]), 11)
Max_pair_as_words ========== (["on"], ["that"])

Iteration number:========== 485
Max pair !!!========== (([30], [6]), 11)
Max_pair_as_words ========== (["out"], ["his"])

Iteration number:========== 486
Max pair !!!========== (([5], [454]), 11)
Max_pair_as_words ========== (["in"], ["fact"])

Iteration number:========== 487
Max pair !!!========== (([14], [41]), 11)
Max_pair_as_words ========== (["for"], ["them"])

Iteration number:========== 488
Max pair !!!========== (([179], [33]), 11)
Max_pair_as_words ========== (["I’m"], ["not"])

Iteration number:========== 489
Max pair !!!========== (([197], [373]), 11)
Max_pair_as_words ========== (["How"], ["many"])

Iteration number:========== 490
Max pair !!!========== (([8], [667]), 11)
Max_pair_as_words ========== (["I"], ["couldn’t"])

Iteration number:========== 491
Max pair !!!========== (([64], [159]), 11)
Max_pair_as_words ========== (["Stephen"], ["Dedalus"])

Iteration number:========== 492
Max pair !!!========== (([1], [333]), 11)
Max_pair_as_words ========== (["of"], ["Ireland"])

Iteration number:========== 493
Max pair !!!========== (([14], [1255]), 11)
Max_pair_as_words ========== (["for"], ["instance"])

Iteration number:========== 494
Max pair !!!========== (([4], [226]), 11)
Max_pair_as_words ========== (["to"], ["give"])

Iteration number:========== 495
Max pair !!!========== (([9], [35]), 11)
Max_pair_as_words ========== (["that"], ["they"])

Iteration number:========== 496
Max pair !!!========== (([223], [3]), 11)
Max_pair_as_words ========== (["such"], ["a"])

Iteration number:========== 497
Max pair !!!========== (([197], [165]), 11)
Max_pair_as_words ========== (["How"], ["much"])

Iteration number:========== 498
Max pair !!!========== (([394], [1]), 11)
Max_pair_as_words ========== (["point"], ["of"])

Iteration number:========== 499
Max pair !!!========== (([67], [3]), 11)
Max_pair_as_words ========== (["What"], ["a"])

Iteration number:========== 500
Max pair !!!========== (([844], [0]), 11)
Max_pair_as_words ========== (["amid"], ["the"])

Iteration number:========== 501
Max pair !!!========== (([11], [29]), 11)
Max_pair_as_words ========== (["it"], ["be"])

Iteration number:========== 502
Max pair !!!========== (([4], [39]), 11)
Max_pair_as_words ========== (["to"], ["their"])

Iteration number:========== 503
Max pair !!!========== (([2], [588]), 11)
Max_pair_as_words ========== (["and"], ["six"])

Iteration number:========== 504
Max pair !!!========== (([624], [748]), 11)
Max_pair_as_words ========== (["Tom"], ["Kernan"])

Iteration number:========== 505
Max pair !!!========== (([13], [56]), 11)
Max_pair_as_words ========== (["on"], ["your"])

Iteration number:========== 506
Max pair !!!========== (([25], [8]), 11)
Max_pair_as_words ========== (["as"], ["I"])

Iteration number:========== 507
Max pair !!!========== (([18], [5]), 11)
Max_pair_as_words ========== (["is"], ["in"])

Iteration number:========== 508
Max pair !!!========== (([1475], [1481]), 11)
Max_pair_as_words ========== (["Bantam"], ["Lyons"])

Iteration number:========== 509
Max pair !!!========== (([16], [167]), 11)
Max_pair_as_words ========== (["you"], ["must"])

Iteration number:========== 510
Max pair !!!========== (([31], [5, 0]), 11)
Max_pair_as_words ========== (["up"], ["in", "the"])

Iteration number:========== 511
Max pair !!!========== (([109], [3]), 11)
Max_pair_as_words ========== (["through"], ["a"])

Iteration number:========== 512
Max pair !!!========== (([196], [3]), 11)
Max_pair_as_words ========== (["took"], ["a"])

Iteration number:========== 513
Max pair !!!========== (([57], [33]), 11)
Max_pair_as_words ========== (["if"], ["not"])

Iteration number:========== 514
Max pair !!!========== (([444], [9]), 11)
Max_pair_as_words ========== (["Like"], ["that"])

Iteration number:========== 515
Max pair !!!========== (([175], [0]), 11)
Max_pair_as_words ========== (["saw"], ["the"])

Iteration number:========== 516
Max pair !!!========== (([2], [9]), 11)
Max_pair_as_words ========== (["and"], ["that"])

Iteration number:========== 517
Max pair !!!========== (([181], [0]), 11)
Max_pair_as_words ========== (["Then"], ["the"])

Iteration number:========== 518
Max pair !!!========== (([0], [127]), 11)
Max_pair_as_words ========== (["the"], ["name"])

Iteration number:========== 519
Max pair !!!========== (([151], [4]), 11)
Max_pair_as_words ========== (["come"], ["to"])

Iteration number:========== 520
Max pair !!!========== (([0], [97]), 11)
Max_pair_as_words ========== (["the"], ["way"])

Iteration number:========== 521
Max pair !!!========== (([1179], [16]), 11)
Max_pair_as_words ========== (["Would"], ["you"])

Iteration number:========== 522
Max pair !!!========== (([197], [74, 16]), 11)
Max_pair_as_words ========== (["How"], ["do", "you"])

Iteration number:========== 523
Max pair !!!========== (([2], [4]), 11)
Max_pair_as_words ========== (["and"], ["to"])

Iteration number:========== 524
Max pair !!!========== (([44], [1]), 11)
Max_pair_as_words ========== (["one"], ["of"])

Iteration number:========== 525
Max pair !!!========== (([751], [16]), 11)
Max_pair_as_words ========== (["Will"], ["you"])

Iteration number:========== 526
Max pair !!!========== (([427], [103]), 11)
Max_pair_as_words ========== (["Let"], ["us"])

Iteration number:========== 527
Max pair !!!========== (([2111], [2110]), 11)
Max_pair_as_words ========== (["CISSY"], ["CAFFREY"])

Iteration number:========== 528
Max pair !!!========== (([0], [78]), 11)
Max_pair_as_words ========== (["the"], ["time"])

Iteration number:========== 529
Max pair !!!========== (([246], [12]), 11)
Max_pair_as_words ========== (["There"], ["was"])

Iteration number:========== 530
Max pair !!!========== (([4], [178]), 11)
Max_pair_as_words ========== (["to"], ["look"])

Iteration number:========== 531
Max pair !!!========== (([389], [7]), 11)
Max_pair_as_words ========== (["Did"], ["he"])

Iteration number:========== 532
Max pair !!!========== (([20], [846]), 11)
Max_pair_as_words ========== (["The"], ["carriage"])

Iteration number:========== 533
Max pair !!!========== (([83], [2]), 11)
Max_pair_as_words ========== (["eyes"], ["and"])

Iteration number:========== 534
Max pair !!!========== (([6], [320]), 11)
Max_pair_as_words ========== (["his"], ["lips"])

Iteration number:========== 535
Max pair !!!========== (([2], [15]), 11)
Max_pair_as_words ========== (["and"], ["her"])

Iteration number:========== 536
Max pair !!!========== (([11], [5, 0]), 11)
Max_pair_as_words ========== (["it"], ["in", "the"])

Iteration number:========== 537
Max pair !!!========== (([246], [18]), 11)
Max_pair_as_words ========== (["There"], ["is"])

Iteration number:========== 538
Max pair !!!========== (([7], [141]), 10)
Max_pair_as_words ========== (["he"], ["came"])

Iteration number:========== 539
Max pair !!!========== (([20], [68]), 10)
Max_pair_as_words ========== (["The"], ["man"])

Iteration number:========== 540
Max pair !!!========== (([1], [88]), 10)
Max_pair_as_words ========== (["of"], ["its"])

Iteration number:========== 541
Max pair !!!========== (([2, 1, 0], [1721, 1]), 10)
Max_pair_as_words ========== (["and", "of", "the"], ["tribe", "of"])

Iteration number:========== 542
Max pair !!!========== (([45], [755]), 10)
Max_pair_as_words ========== (["an"], ["instant"])

Iteration number:========== 543
Max pair !!!========== (([9], [8]), 10)
Max_pair_as_words ========== (["that"], ["I"])

Iteration number:========== 544
Max pair !!!========== (([2], [182]), 10)
Max_pair_as_words ========== (["and"], ["white"])

Iteration number:========== 545
Max pair !!!========== (([153], [68]), 10)
Max_pair_as_words ========== (["young"], ["man"])

Iteration number:========== 546
Max pair !!!========== (([12], [7]), 10)
Max_pair_as_words ========== (["was"], ["he"])

Iteration number:========== 547
Max pair !!!========== (([18], [6]), 10)
Max_pair_as_words ========== (["is"], ["his"])

Iteration number:========== 548
Max pair !!!========== (([6], [172]), 10)
Max_pair_as_words ========== (["his"], ["voice"])

Iteration number:========== 549
Max pair !!!========== (([81], [15]), 10)
Max_pair_as_words ========== (["into"], ["her"])

Iteration number:========== 550
Max pair !!!========== (([106], [107]), 10)
Max_pair_as_words ========== (["They"], ["say"])

Iteration number:========== 551
Max pair !!!========== (([160], [1008]), 10)
Max_pair_as_words ========== (["Mrs"], ["Marion"])

Iteration number:========== 552
Max pair !!!========== (([125], [6]), 10)
Max_pair_as_words ========== (["under"], ["his"])

Iteration number:========== 553
Max pair !!!========== (([23], [407]), 10)
Max_pair_as_words ========== (["He"], ["knows"])

Iteration number:========== 554
Max pair !!!========== (([907], [1233]), 10)
Max_pair_as_words ========== (["George"], ["Lidwell"])

Iteration number:========== 555
Max pair !!!========== (([11], [3]), 10)
Max_pair_as_words ========== (["it"], ["a"])

Iteration number:========== 556
Max pair !!!========== (([34], [3]), 10)
Max_pair_as_words ========== (["had"], ["a"])

Iteration number:========== 557
Max pair !!!========== (([1053], [29]), 10)
Max_pair_as_words ========== (["Might"], ["be"])

Iteration number:========== 558
Max pair !!!========== (([188], [8]), 10)
Max_pair_as_words ========== (["am"], ["I"])

Iteration number:========== 559
Max pair !!!========== (([329], [0]), 10)
Max_pair_as_words ========== (["It’s"], ["the"])

Iteration number:========== 560
Max pair !!!========== (([8], [262]), 10)
Max_pair_as_words ========== (["I"], ["thought"])

Iteration number:========== 561
Max pair !!!========== (([724], [2294]), 10)
Max_pair_as_words ========== (["Paddy"], ["Leonard"])

Iteration number:========== 562
Max pair !!!========== (([656], [4]), 10)
Max_pair_as_words ========== (["trying"], ["to"])

Iteration number:========== 563
Max pair !!!========== (([23], [439]), 10)
Max_pair_as_words ========== (["He"], ["held"])

Iteration number:========== 564
Max pair !!!========== (([51], [0]), 10)
Max_pair_as_words ========== (["which"], ["the"])

Iteration number:========== 565
Max pair !!!========== (([72], [34]), 10)
Max_pair_as_words ========== (["who"], ["had"])

Iteration number:========== 566
Max pair !!!========== (([20], [740]), 10)
Max_pair_as_words ========== (["The"], ["priest"])

Iteration number:========== 567
Max pair !!!========== (([190], [99]), 10)
Max_pair_as_words ========== (["poor"], ["little"])

Iteration number:========== 568
Max pair !!!========== (([47], [3]), 10)
Max_pair_as_words ========== (["And"], ["a"])

Iteration number:========== 569
Max pair !!!========== (([1], [126]), 10)
Max_pair_as_words ========== (["of"], ["any"])

Iteration number:========== 570
Max pair !!!========== (([161], [29]), 10)
Max_pair_as_words ========== (["To"], ["be"])

Iteration number:========== 571
Max pair !!!========== (([59], [68]), 10)
Max_pair_as_words ========== (["old"], ["man"])

Iteration number:========== 572
Max pair !!!========== (([702], [7]), 10)
Max_pair_as_words ========== (["Was"], ["he"])

Iteration number:========== 573
Max pair !!!========== (([109], [6]), 10)
Max_pair_as_words ========== (["through"], ["his"])

Iteration number:========== 574
Max pair !!!========== (([4], [645]), 10)
Max_pair_as_words ========== (["to"], ["meet"])

Iteration number:========== 575
Max pair !!!========== (([55], [293]), 10)
Max_pair_as_words ========== (["says"], ["Lenehan"])

Iteration number:========== 576
Max pair !!!========== (([194], [0]), 10)
Max_pair_as_words ========== (["behind"], ["the"])

Iteration number:========== 577
Max pair !!!========== (([2], [80]), 10)
Max_pair_as_words ========== (["and"], ["two"])

Iteration number:========== 578
Max pair !!!========== (([171], [33]), 10)
Max_pair_as_words ========== (["though"], ["not"])

Iteration number:========== 579
Max pair !!!========== (([2231], [2200]), 10)
Max_pair_as_words ========== (["quaker"], ["librarian"])

Iteration number:========== 580
Max pair !!!========== (([2], [5]), 10)
Max_pair_as_words ========== (["and"], ["in"])

Iteration number:========== 581
Max pair !!!========== (([5], [733]), 10)
Max_pair_as_words ========== (["in"], ["front"])

Iteration number:========== 582
Max pair !!!========== (([241], [8]), 10)
Max_pair_as_words ========== (["If"], ["I"])

Iteration number:========== 583
Max pair !!!========== (([6], [164]), 10)
Max_pair_as_words ========== (["his"], ["left"])

Iteration number:========== 584
Max pair !!!========== (([75], [7]), 10)
Max_pair_as_words ========== (["would"], ["he"])

Iteration number:========== 585
Max pair !!!========== (([34], [7]), 10)
Max_pair_as_words ========== (["had"], ["he"])

Iteration number:========== 586
Max pair !!!========== (([7], [79]), 10)
Max_pair_as_words ========== (["he"], ["did"])

Iteration number:========== 587
Max pair !!!========== (([98], [3]), 10)
Max_pair_as_words ========== (["has"], ["a"])

Iteration number:========== 588
Max pair !!!========== (([768], [117]), 10)
Max_pair_as_words ========== (["OF"], ["THE"])

Iteration number:========== 589
Max pair !!!========== (([622], [0]), 10)
Max_pair_as_words ========== (["within"], ["the"])

Iteration number:========== 590
Max pair !!!========== (([19], [17]), 10)
Max_pair_as_words ========== (["at"], ["him"])

Iteration number:========== 591
Max pair !!!========== (([64], [146]), 10)
Max_pair_as_words ========== (["Stephen"], ["asked"])

Iteration number:========== 592
Max pair !!!========== (([94], [84]), 10)
Max_pair_as_words ========== (["You"], ["will"])

Iteration number:========== 593
Max pair !!!========== (([8], [613]), 10)
Max_pair_as_words ========== (["I"], ["can’t"])

Iteration number:========== 594
Max pair !!!========== (([30], [2]), 10)
Max_pair_as_words ========== (["out"], ["and"])

Iteration number:========== 595
Max pair !!!========== (([343], [111]), 10)
Max_pair_as_words ========== (["Good"], ["day"])

Iteration number:========== 596
Max pair !!!========== (([117], [2295]), 9)
Max_pair_as_words ========== (["THE"], ["MOTHER"])

Iteration number:========== 597
Max pair !!!========== (([0], [327]), 9)
Max_pair_as_words ========== (["the"], ["second"])

Iteration number:========== 598
Max pair !!!========== (([515], [4, 42]), 9)
Max_pair_as_words ========== (["ought"], ["to", "have"])

Iteration number:========== 599
Max pair !!!========== (([700], [28]), 9)
Max_pair_as_words ========== (["Tell"], ["me"])

~>/entropy_tokenizer


```




