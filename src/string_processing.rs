// functions for string processing 

pub fn separate_punctuation<'a>(strng:&'a mut str, st:&str) -> &'a str{
    for ch in st.chars() {
        strng.replace(ch, " ");
    }
    
    return strng
}  

