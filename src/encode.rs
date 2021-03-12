use crate::types::{Arena, Huffman};
use std::string::String;
use std::{char, collections::HashMap};

pub fn encode(input: &String, tree: &Huffman, arena: &Arena) -> String {
    let char_map = build_char_map(tree, arena);
    for i in input.chars() {}
    "hello".to_string()
}

fn get_path_from_char<'a>(tree: &Huffman, char_map: HashMap<char, usize>, arena: &Arena) {}

fn build_char_map(tree: &Huffman, arena: &Arena) -> HashMap<char, usize> {
    let mut char_map = HashMap::new();
    fn _build_char_map(tree: &Huffman, char_map: &mut HashMap<char, usize>, arena: &Arena) {
        for child in tree.children.clone().unwrap().iter() {
            match arena[*child].character {
                Some(character) => {
                    char_map.insert(character, *child);
                    ()
                }
                None => _build_char_map(&arena[*child], char_map, arena),
            }
        }
    }
    _build_char_map(tree, &mut char_map, arena);
    char_map
}
