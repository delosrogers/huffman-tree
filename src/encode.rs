use crate::types::{CharCount, HuffNode, Huffman};
use std::collections::HashMap;
use std::string::String;

pub fn encode(input: &String, tree: &Huffman) -> String {
    let char_map = build_char_map(&tree);
    for i in input.chars() {}
    "hello".to_string()
}

fn get_path_from_char<'a>(tree: &'a Huffman, char_map: HashMap<char, &'a HuffNode>) {}

fn build_char_map<'a>(tree: &'a Huffman) -> HashMap<char, Box<HuffNode>> {
    let mut char_map = HashMap::new();
    fn _build_char_map<'a>(tree: &'a Huffman, char_map: &mut HashMap<char, Box<HuffNode>>) {
        for child in tree.children.clone().iter() {
            match **child {
                HuffNode::Huff(huffman) => {
                    _build_char_map(&huffman, char_map);
                }
                HuffNode::CharCount(charcount) => {
                    char_map.insert(charcount.character, *child);
                }
            };
        
    }
    _build_char_map(&tree, &mut char_map);
    char_map
}
