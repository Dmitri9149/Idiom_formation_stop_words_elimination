// vocab of pairs, where pairs are pairs of words 
// and where words are represented as u32 indexes
pub struct Pairs {
    pub pairs:Vec<((u32,u32),u32>
}

impl Pairs {
    pub fn new_with_capacity(capasity:u32) -> Pairs {
        Pairs {pairs:Vec<((u32,u32),u32)>::with_capacity(capasity)}
    }

    pub fn new() -> Pairs {
        Pairs{pairs:<Vec((u32,u32),u32)>}
}
