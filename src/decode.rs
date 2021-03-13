use crate::types::{Arena, Huffman};
use std::string::String;

pub fn decode(input: &Vec<u8>, tree: &Huffman, arena: &Arena) -> String {
    let bits = make_bit_list(input);
    let mut current_tree_position = tree;
    let mut decoded_chars = Vec::new();
    for bit in bits {
        match &current_tree_position.clone().children {
            Some(children) => {
                current_tree_position = &arena[children[bit]];
                match &current_tree_position.clone().children {
                    Some(_children) => (),
                    None => {
                        decoded_chars.push(current_tree_position.clone().character.unwrap());
                        current_tree_position = tree;
                    }
                }
            }
            None => {
                panic!("should not have iterated if didn't have child")
            }
        }
    }
    let mut decoded_string = String::new();
    for ch in decoded_chars {
        decoded_string.push(ch);
    }
    decoded_string
}

fn make_bit_list(bytes: &Vec<u8>) -> Vec<usize> {
    let mut res = Vec::new();
    for b in bytes.iter() {
        let mut byte = *b;
        for _i in 0..8 {
            res.push((byte % 2) as usize);
            byte = byte >> 1;
        }
    }
    res
}

#[test]
fn test_make_bit_list() {
    let bytes = vec![248, 1];
    let test_bits = make_bit_list(&bytes);
    assert_eq!(
        test_bits,
        vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]
    )
}

#[test]
fn test_decode() {
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
    let res = decode(&vec![248 as u8], &arena[0], &arena);
    assert_eq!(res, "hell".to_string())
}
