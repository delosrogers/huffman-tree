
use std::fmt;
#[derive(Debug)]
pub struct Huffman {
    pub count: i32,
    pub children: Vec<HuffNode>,
}
#[derive(Debug)]
pub struct CharCount {
    pub count: i32,
    pub character: char,
}

impl Clone for CharCount {
    fn clone(&self) -> Self {
        CharCount {
            count: self.count,
            character: self.character,
        }
    }
}
#[derive(Debug)]
pub enum HuffNode {
    Huff(Huffman),
    CharCount(CharCount),
}

impl Eq for HuffNode {}

impl HuffNode {
    pub fn count(&self) -> i32 {
        match self {
            HuffNode::Huff(huffman) => huffman.count,
            HuffNode::CharCount(charcount) => charcount.count,
        }
    }
}
impl PartialOrd for HuffNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffNode {
    fn eq(&self, other: &Self) -> bool {
        self.count() == other.count()
    }
}

impl Ord for HuffNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count().cmp(&other.count())
    }
}

impl fmt::Display for Huffman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}
impl fmt::Display for HuffNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CharCount(charcount) => write!(f, "{}", charcount),
            Self::Huff(huffman) => write!(f, "{}", huffman),
        }
    }
}
impl fmt::Display for CharCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.count, self.character)
    }
}
