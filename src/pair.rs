// implements structure Token which keeps string "token" , number of the 
// tokens and numerical equivalent 
use std::cmp::Ordering;


pub struct Pair {
    tensor:Vec<Vec<u32>>
}

impl Pair {
    pub fn new() -> {
        Tensor {
            tensor:Vec<_>::new(),
        }
    }
}
/*
impl Ord for Tensor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for Tensor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tensor {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}
*/
