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



