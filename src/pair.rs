// implements structure Token which keeps string "token" , number of the 
// tokens and numerical equivalent 
use std::cmp::Ordering;


pub struct Pair {
    first:u32,
    second:u32,
    frequency: u32,
}

impl Pair {
    pub fn new(first:u32, second:u32, freq:u32) -> {
        Pair {
            first:first,
            second:second,
            frequency:freq
        }
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}
