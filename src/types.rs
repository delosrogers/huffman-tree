use std::fmt;

#[derive(Debug)]
pub struct Huffman {
    pub count: i32,
    pub children: Vec<Box<HuffNode>>,
    pub parent: Option<Box<Huffman>>,
}
#[derive(Debug)]
pub struct CharCount {
    pub count: i32,
    pub character: char,
    pub parent: Box<Huffman>,
}

impl Clone for CharCount {
    fn clone(&self) -> Self {
        CharCount {
            count: self.count,
            character: self.character,
            parent: self.parent,
        }
    }
}
#[derive(Debug)]
pub enum HuffNode {
    Huff(Huffman),
    CharCount(CharCount),
}

// impl HuffNode {
//     pub fn children(&self, arena: &Arena) -> Option<Vec<HuffNode>> {
//         let vec = Vec::new();
//         match self {
//             HuffNode::Huff(huffman) => {
//                 for i in huffman.children.iter() {
//                     vec.push(arena.arena[*i]);
//                 }
//                 Some(vec)
//             }
//             HuffNode::CharCount(charcount) => None,
//         }
//     }

//     pub fn parent(&self, arena: &Arena) -> Option<HuffNode> {
//         match self {
//             Self::CharCount(charcount) => Some(arena.arena[charcount.parent]),
//             Self::Huff(huffman) => match huffman.parent {
//                 Some(idx) => Some(arena.arena[idx]),
//                 None => None,
//             },
//         }
//     }
// }

// impl Eq for HuffNode {}

impl HuffNode {
    pub fn count(&self) -> i32 {
        match self {
            HuffNode::Huff(huffman) => huffman.count,
            HuffNode::CharCount(charcount) => charcount.count,
        }
    }

    pub fn set_parent(&mut self, parent: Box<Huffman>) {
        match self {
            HuffNode::Huff(huffman) => huffman.parent = Some(parent),
            HuffNode::CharCount(charcount) => charcount.parent = parent,
        }
    }
}

impl Eq for HuffNode {}

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
