// functions for string processing 
// separate a punctuation symbol from the list 'st' by " " from both sides 
pub fn separate_punctuation(strng:String, st:&str) -> String{
    let mut res=strng;
    for ch in st.chars() {
        res = res
            .replace(&ch.to_string(), &[" ", &ch.to_string(), " "].join(""))
            .to_owned();
    }
    res
}
// split a string on space to get a collection of words
pub fn to_collection_split_on_space(st:String) -> Vec<String>{
    let res = st
        .split(" ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    res
}

