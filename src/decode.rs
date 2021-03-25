use crate::types::{ProdArena, ProdHuffman};
use rayon::prelude::*;
use std::string::String;
use std::time::Instant;

pub fn decode(input: &Vec<u8>, tree: &ProdHuffman, arena: &ProdArena) -> String {
    // let bits = make_bit_list(input);
    let mut current_tree_position = tree;
    let mut decoded_chars = Vec::new();
    for b in input {
        let mut byte = *b;
        for _i in 0..8 {
            let bit = byte % 2;
            byte = byte >> 1;
            match &current_tree_position.children {
                Some(children) => {
                    current_tree_position = &arena[children[bit as usize]];
                    if current_tree_position.children.is_none() {
                        decoded_chars.push(*current_tree_position.character.as_ref().unwrap());
                        current_tree_position = tree;
                    }
                }
                None => {
                    panic!("should not have iterated if didn't have child")
                }
            }
        }
    }
    let mut decoded_string = String::new();
    for ch in decoded_chars {
        decoded_string.push(ch);
    }
    decoded_string
}

// fn make_bit_list(bytes: &Vec<u8>) -> Vec<u8> {
//     let mut res = Vec::new();
//     let now = Instant::now();
//     for b in bytes.iter() {
//         let mut byte = *b;
//         for _i in 0..8 {
//             res.push((byte % 2) as u8);
//             byte = byte >> 1;
//         }
//     }
//     println!("time to make bit list {:?}", now.elapsed());
//     res
// }

// #[test]
// fn test_make_bit_list() {
//     let bytes = vec![248, 1];
//     let test_bits = make_bit_list(&bytes);
//     assert_eq!(
//         test_bits,
//         vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]
//     )
// }

// #[test]
// fn test_decode() {
//     let arena = vec![
//         Huffman {
//             count: 5,
//             children: Some(vec![5, 6]),
//             parent: None,
//             character: None,
//         },
//         Huffman {
//             count: 1,
//             children: None,
//             parent: Some(5),
//             character: Some('h'),
//         },
//         Huffman {
//             count: 2,
//             children: None,
//             parent: Some(6),
//             character: Some('l'),
//         },
//         Huffman {
//             count: 1,
//             children: None,
//             parent: Some(5),
//             character: Some('e'),
//         },
//         Huffman {
//             count: 1,
//             children: None,
//             parent: Some(6),
//             character: Some('o'),
//         },
//         Huffman {
//             count: 2,
//             children: Some(vec![1, 3]),
//             parent: Some(0),
//             character: None,
//         },
//         Huffman {
//             count: 3,
//             children: Some(vec![4, 2]),
//             parent: Some(0),
//             character: None,
//         },
//     ];
//     let res = decode(&vec![248 as u8], &arena[0], &arena);
//     assert_eq!(res, "hell".to_string())
// }
