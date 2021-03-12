use std::fmt;

pub type Arena = Vec<Huffman>;
#[derive(Debug)]
pub struct Huffman {
    pub count: i32,
    pub children: Option<Vec<usize>>,
    pub parent: Option<usize>,
    pub character: Option<char>,
}

impl fmt::Display for Huffman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.character {
            Some(character) => write!(f, "{}: {}", character, self.count),
            None => write!(f, "{}", self.count),
        }
    }
}

pub fn compare_usize_pointers(first: &usize, second: &usize, arena: &Arena) -> std::cmp::Ordering {
    arena[*first].count.cmp(&arena[*second].count)
}
