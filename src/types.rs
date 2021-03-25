use rayon::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

pub type Arena = Vec<Huffman>;
#[derive(Debug, Clone)]
pub struct Huffman {
    pub count: i32,
    pub children: Option<Vec<usize>>,
    pub parent: Option<usize>,
    pub character: Option<char>,
}

pub type ProdArena = Vec<ProdHuffman>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProdHuffman {
    pub children: Option<[usize; 2]>,
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
impl Huffman {
    fn into_prod(&self) -> ProdHuffman {
        let mut prod_huffman = ProdHuffman {
            children: None,
            parent: self.parent,
            character: self.character,
        };
        match self.children.as_ref() {
            Some(children) => prod_huffman.children = Some([children[0], children[1]]),
            None => (),
        }
        prod_huffman
    }
}

pub fn into_prod(arena: &Arena) -> ProdArena {
    arena.into_par_iter().map(|huff| huff.into_prod()).collect()
}
