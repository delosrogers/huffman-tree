mod tree;
mod types;
use crate::tree::{make_start_count_huffman, make_start_count_huffman_with_hash_map, make_tree};
use crate::types::HuffNode;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::string::String;
use std::time::Instant;

fn pprint_huffman(node: &HuffNode) {
    fn pprint_huffman(node: &HuffNode, prefix: String, last: bool) {
        let prefix_current = if last { "'- " } else { "|- " };
        println!("{}{}{}", prefix, prefix_current, node);
        let prefix_child = if last { "   " } else { "|   " };
        let prefix = prefix + prefix_child;
        match node {
            HuffNode::Huff(huffman) => {
                let last_child = huffman.children.len() - 1;
                for (i, child) in huffman.children.iter().enumerate() {
                    pprint_huffman(&child, prefix.to_string(), i == last_child);
                }
            }
            _ => (),
        };
    }
    pprint_huffman(node, "".to_string(), true);
}

fn main() {
    // let input = thread_rng()
    //     .sample_iter(&Alphanumeric)
    //     .take(1000)
    //     .map(char::from)
    //     .collect();
    let input = std::fs::read_to_string("file.txt").expect("something went wrong");
    let mut now = Instant::now();
    // for _i in 0..1000 {
    let mut huffman = make_start_count_huffman(&input);
    // println!("{:?}", huffman);
    let mut _result = make_tree(&mut huffman);
    // }
    println!("{:?}", now.elapsed());
    pprint_huffman(&HuffNode::Huff(huffman));
    now = Instant::now();
    // println!("{:?}", *result);
    // for _i in 0..1000 {
    let mut huffman = make_start_count_huffman_with_hash_map(&input);
    let mut _result = make_tree(&mut huffman);
    // }
    println!("{:?}", now.elapsed());
    pprint_huffman(&HuffNode::Huff(huffman));
}
