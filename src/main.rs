#[macro_use]
extern crate partial_application;
mod decode;
mod encode;
mod tree;
mod types;
use crate::decode::decode;
use crate::tree::{make_start_count_huffman_with_hash_map, make_tree};
use crate::types::{Arena, Huffman};
use encode::encode;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::string::String;
use std::time::Instant;

fn pprint_huffman(tree: &Huffman, arena: &Arena) {
    fn _pprint_huffman(node: &Huffman, prefix: String, last: bool, arena: &Arena) {
        let prefix_current = if last { "'- " } else { "|- " };
        println!("{}{}{}", prefix, prefix_current, node);
        let prefix_child = if last { "   " } else { "|   " };
        let prefix = prefix + prefix_child;
        match node.children.clone() {
            Some(children) => {
                let last_child = children.len() - 1;
                for (i, child) in children.iter().enumerate() {
                    _pprint_huffman(&arena[*child], prefix.to_string(), i == last_child, arena);
                }
            }
            None => (),
        };
    }
    _pprint_huffman(tree, "".to_string(), false, arena)
}

fn main() {
    // let input = thread_rng()
    //     .sample_iter(&Alphanumeric)
    //     .take(1000)
    //     .map(char::from)
    //     .collect();
    let input = std::fs::read_to_string("file.txt").expect("something went wrong");
    // let mut now = Instant::now();
    // // for _i in 0..1000 {
    // let mut huffman = make_start_count_huffman(&input);
    // // println!("{:?}", huffman);
    // let mut _result = make_tree(&mut huffman);
    // // }
    // println!("{:?}", now.elapsed());
    // pprint_huffman(&HuffNode::Huff(huffman));
    let now = Instant::now();
    // println!("{:?}", *result);
    // for _i in 0..1000 {
    let mut start_state = make_start_count_huffman_with_hash_map(&input);
    let mut huffman = start_state.0;
    let mut arena: Arena = start_state.1;
    let mut _result = make_tree(huffman, &mut arena);
    // }
    println!("{:?}", now.elapsed());
    pprint_huffman(&arena[huffman], &arena);
    let encoded = encode(&input, &arena[0], &arena);
    std::fs::write("compressed.txt", &encoded[..]);
    let decoded = decode(&encoded, &arena[0], &arena);
    std::fs::write("de-compressed.txt", &decoded[..]);
}
