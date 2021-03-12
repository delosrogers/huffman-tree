use crate::types::{CharCount, HuffNode, Huffman};
use std::{borrow::Borrow, collections::HashMap};
/// Makes a non valid Huffman tree that contains the counts of every
/// character
pub fn make_start_count_huffman_with_hash_map(string: &String) -> Box<Huffman> {
    let input_array = string.chars();
    let mut letters_map: HashMap<char, i32> = HashMap::new();
    for i in input_array {
        match letters_map.get(&i) {
            Some(count) => letters_map.insert(i, *count + 1),
            None => letters_map.insert(i, 1),
        };
    }
    let mut letters = Vec::new();
    let mut total = 0;
    let mut huffman = Box::new(Huffman {
        count: 0,
        children: vec![],
        parent: None,
    });
    for (character, count) in letters_map.iter() {
        total += count;
        letters.push(Box::new(HuffNode::CharCount(CharCount {
            count: *count,
            character: *character,
            parent: huffman,
        })));
    }
    huffman.children = letters;
    huffman.count = total;
    huffman
}

/// makes a non valid Huffman tree that contains the counts
/// of every character without using a hashmap (slow)
// pub fn make_start_count_huffman(string: &String) -> Huffman {
//     let input_array = string.chars();
//     let mut letters: Vec<CharCount> = Vec::new();
//     for i in input_array.clone() {
//         increment_char(&mut letters, i);
//     }
//     let mut total: i32 = 0;
//     for i in letters.clone() {
//         total += i.count;
//     }
//     let mut huff_node_letters: Vec<HuffNode> = Vec::new();
//     for i in letters {
//         huff_node_letters.push(HuffNode::CharCount(i));
//     }
//     Huffman {
//         count: total,
//         children: huff_node_letters,
//     }
// }

// ///increments characters from the non make_start_count_huffman()
// fn increment_char(char_count_vec: &mut Vec<CharCount>, k: char) {
//     for i in 0..(char_count_vec.clone().len() as isize) {
//         if char_count_vec[i as usize].character == k {
//             // println!("{:?} increment char", char_count_vec[i as usize]);
//             char_count_vec[i as usize].count += 1;
//             return;
//         }
//     }
//     char_count_vec.push(CharCount {
//         count: 1,
//         character: k,
//     });
// }

/// taking the non valid huffman tree that just has char counts
/// mutates it into a valid one
pub fn make_tree(huffman: Box<Huffman>) {
    if huffman.children.len() > 2 {
        huffman.children.sort();
        let parent = Huffman {
            count: 0,
            children: vec![],
            parent: Some(huffman),
        };
        huffman.children.push(Box::new(HuffNode::Huff(parent)));
        let parent_box = Box::new(parent);
        let mut smallest = huffman.children.remove(0);
        smallest.set_parent(parent_box);
        let mut second_smallest = huffman.children.remove(0);
        second_smallest.set_parent(parent_box);
        huffman.children.push(Box::new(HuffNode::Huff(Huffman {
            count: smallest.count() + second_smallest.count(),
            children: vec![smallest, second_smallest],
            parent: Some(huffman),
        })));
        make_tree(huffman)
    }
}
