// functions for string processing 

pub fn separate_punctuation(strng:String, st:&str) -> String{
    let mut res=strng;
    for ch in st.chars() {
        res = res
            .replace(&ch.to_string(), &[" ", &ch.to_string(), " "].join(""))
            .to_owned();
    }
    res
}

