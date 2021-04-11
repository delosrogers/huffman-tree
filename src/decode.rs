use crate::types::{ProdArena, ProdHuffman};
use std::string::String;

pub fn decode(input: &Vec<u8>, tree: &ProdHuffman, arena: &ProdArena) -> String {
    // let bits = make_bit_list(input);
    let mut current_tree_position = tree;
    let mut decoded_string = String::new();
    for b in input {
        let mut byte = *b;
        for _i in 0..8 {
            let bit = byte % 2;
            byte = byte >> 1;
            match &current_tree_position.children {
                Some(children) => {
                    current_tree_position = &arena[children[bit as usize]];
                    if current_tree_position.children.is_none() {
                        decoded_string.push(*current_tree_position.character.as_ref().unwrap());
                        current_tree_position = tree;
                    }
                }
                None => {
                    panic!("should not have iterated if didn't have child")
                }
            }
        }
    }
    decoded_string
}

#[test]
fn test_decode() {
    let arena = vec![
        ProdHuffman {
            children: Some([5, 6]),
            parent: None,
            character: None,
        },
        ProdHuffman {
            children: None,
            parent: Some(5),
            character: Some('h'),
        },
        ProdHuffman {
            children: None,
            parent: Some(6),
            character: Some('l'),
        },
        ProdHuffman {
            children: None,
            parent: Some(5),
            character: Some('e'),
        },
        ProdHuffman {
            children: None,
            parent: Some(6),
            character: Some('o'),
        },
        ProdHuffman {
            children: Some([1, 3]),
            parent: Some(0),
            character: None,
        },
        ProdHuffman {
            children: Some([4, 2]),
            parent: Some(0),
            character: None,
        },
    ];
    let res = decode(&vec![248 as u8], &arena[0], &arena);
    assert_eq!(res, "hell".to_string())
}
