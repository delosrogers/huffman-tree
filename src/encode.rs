use crate::types::{Arena, Huffman};
use std::string::String;
use std::{char, collections::HashMap};

pub fn encode(input: &String, tree: &Huffman, arena: &Arena) -> Vec<u8> {
    let char_map = build_char_map(tree, arena);
    let mut output_bytes: Vec<u8> = Vec::new();
    let mut current_bit: u8 = 0;
    let mut current_byte: u8 = 0;
    let base: u8 = 2;
    let mut path_for_current = Vec::new();
    println!("{:?}", arena);
    println!("{:?}", tree);
    for character in input.chars() {
        path_for_current = get_path_from_char(character, &char_map, arena);
        for i in path_for_current {
            if current_bit > 7 {
                current_bit = 0;
                output_bytes.push(current_byte);
                current_byte = 0;
            }
            if i {
                current_byte += base.pow(current_bit as u32);
            }
            current_bit += 1;
        }
    }
    output_bytes.push(current_byte);
    output_bytes
}

fn get_path_from_char(
    character: char,
    char_map: &HashMap<char, usize>,
    arena: &Arena,
) -> Vec<bool> {
    let idx = *char_map.get(&character).unwrap();
    let mut path = Vec::new();
    path = go_up_tree(idx, arena, path);
    path.reverse();
    path
}

fn go_up_tree(curr_node: usize, arena: &Arena, mut path: Vec<bool>) -> Vec<bool> {
    let leaf = &arena[curr_node];
    let parent = &arena[leaf.parent.unwrap()];
    let children = parent.children.clone().unwrap();
    let mut first_or_second_child = false;
    for i in 0..2 {
        if children[i] == curr_node {
            first_or_second_child = if i == 1 { true } else { false };
        }
    }
    path.push(first_or_second_child);
    match parent.parent {
        Some(_) => go_up_tree(leaf.parent.unwrap(), arena, path),
        None => path,
    }
}

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

#[test]
fn test_encode() {
    let arena = vec![
        Huffman {
            count: 5,
            children: Some(vec![5, 6]),
            parent: None,
            character: None,
        },
        Huffman {
            count: 1,
            children: None,
            parent: Some(5),
            character: Some('h'),
        },
        Huffman {
            count: 2,
            children: None,
            parent: Some(6),
            character: Some('l'),
        },
        Huffman {
            count: 1,
            children: None,
            parent: Some(5),
            character: Some('e'),
        },
        Huffman {
            count: 1,
            children: None,
            parent: Some(6),
            character: Some('o'),
        },
        Huffman {
            count: 2,
            children: Some(vec![1, 3]),
            parent: Some(0),
            character: None,
        },
        Huffman {
            count: 3,
            children: Some(vec![4, 2]),
            parent: Some(0),
            character: None,
        },
    ];

    let res = encode(&"hello".to_string(), &arena[0], &arena);
    assert_eq!(res, vec![248 as u8, 1 as u8]);
}
