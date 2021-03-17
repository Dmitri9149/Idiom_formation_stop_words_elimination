// implements structure Token which keeps string "token" , number of the 
// tokens and numerical equivalent 
use std::cmp::Ordering;


pub struct Token {
    token:String,
    frequency: u32,
    index:u32
}

impl Token {
    pub fn new(st:&str, freq:u32,index:u32) -> {
        Token {
            token:st.to_string,
            frequency:freq,
            index:index
        }
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequencyy
    }
}
