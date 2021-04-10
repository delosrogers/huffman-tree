use crate::types::{Encoded, ProdArena, ProdHuffman};
use rayon::prelude::*;
use std::string::String;
use std::time::Instant;
use std::{char, collections::HashMap};

const SEGMENT_LEN: usize = 100000;

pub fn encode(input: String, tree: &ProdHuffman, arena: &ProdArena) -> Encoded {
    let split_input = split_string(input);
    let mut split_encoded: Vec<Vec<u8>> = Vec::new();
    let mut segment_lengths: Vec<usize> = Vec::new();
    for i in split_input {
        let encoded_segment = encode_segment(i, tree, arena);
        segment_lengths.push(encoded_segment.len());
        split_encoded.push(encoded_segment);
    }
    let encoded = Encoded {
        SplitLocs: segment_lengths,
        Data: *split_encoded
            .iter_mut()
            .reduce(|existing_data: &mut Vec<u8>, new_segment: &mut Vec<u8>| {
                existing_data.append(new_segment);
                existing_data
            })
            .unwrap(),
    };
    encoded
}

fn split_string(input: String) -> Vec<String> {
    let mut char_vec: Vec<char> = input.chars().collect();
    let mut i: usize = SEGMENT_LEN;
    let mut split_vec = Vec::new();
    while i < char_vec.len() {
        split_vec.push(char_vec.split_off(i));
        i += SEGMENT_LEN;
    }
    split_vec.push(char_vec);
    let res: Vec<String> = split_vec
        .par_iter()
        .map(|chars| chars.iter().collect::<String>())
        .collect();
    res.reverse();
    res
}

fn encode_segment(input: String, tree: &ProdHuffman, arena: &ProdArena) -> Vec<u8> {
    let now = Instant::now();
    let char_map = build_char_map(tree, arena);
    // let mut paths = vec![];
    let directions: Vec<u8> = input
        .chars()
        .collect::<Vec<char>>()
        .par_iter_mut()
        .map(|character| get_path_from_char(*character, &char_map, &arena))
        .fold(
            || Vec::new(),
            |mut directions, mut path| {
                directions.append(&mut path);
                directions
            },
        )
        .reduce(
            || Vec::new(),
            |mut directions, mut sub_dirs| {
                directions.append(&mut sub_dirs);
                directions
            },
        );

    // let mut directions = Vec::new();
    // for i in paths.iter_mut() {
    //     directions.append(i);
    // }

    println!("time to build paths {:?}", now.elapsed());
    let now = Instant::now();
    let mut output_bytes: Vec<u8> = Vec::new();
    let mut current_bit: u8 = 0;
    let mut current_byte: u8 = 0;
    let base: u8 = 2;
    for i in directions {
        if current_bit > 7 {
            current_bit = 0;
            output_bytes.push(current_byte);
            current_byte = 0;
        }

        current_byte += i * (base << current_bit);
        current_bit += 1;
    }
    output_bytes.push(current_byte);
    println!("time to make byte vector {:?}", now.elapsed());
    output_bytes.shrink_to_fit();
    println!("exiting encodev");
    output_bytes
}

fn get_path_from_char(
    character: char,
    char_map: &HashMap<char, usize>,
    arena: &ProdArena,
) -> Vec<u8> {
    let idx = *char_map.get(&character).unwrap();
    let mut path = Vec::with_capacity(10);
    path = go_up_tree(idx, arena, path);
    path.reverse();
    path
}

fn go_up_tree(curr_node: usize, arena: &ProdArena, mut path: Vec<u8>) -> Vec<u8> {
    let leaf = &arena[curr_node];
    let parent = &arena[leaf.parent.unwrap()];
    let children = parent.children.as_ref().unwrap();
    let mut first_or_second_child = 0;
    for i in 0..2 {
        if children[i] == curr_node {
            first_or_second_child = if i == 1 { 1 as u8 } else { 0 as u8 };
        }
    }
    path.push(first_or_second_child);
    if parent.parent.is_some() {
        go_up_tree(leaf.parent.unwrap(), arena, path)
    } else {
        path
    }
}

fn build_char_map(tree: &ProdHuffman, arena: &ProdArena) -> HashMap<char, usize> {
    let mut char_map = HashMap::new();
    fn _build_char_map(tree: &ProdHuffman, char_map: &mut HashMap<char, usize>, arena: &ProdArena) {
        for child in tree.children.as_ref().unwrap().iter() {
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
