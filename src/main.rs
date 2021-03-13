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
use std::io;
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
    println!("enter name of file in ignored directory:");
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("something went wrong reading input");
    file_name = file_name.replace("\r\n", "");
    println!("{:?}", file_name);
    let input = std::fs::read_to_string(&file_name).expect("something went wrong");

    println!("starting code generation");
    let mut now = Instant::now();
    let mut start_state = make_start_count_huffman_with_hash_map(&input);
    let huffman = start_state.0;
    let mut arena: Arena = start_state.1;
    let mut _result = make_tree(huffman, &mut arena);
    let duration = now.elapsed();
    pprint_huffman(&arena[huffman], &arena);
    println!("code generation took: {:?}", duration);

    println!("starting compression");
    now = Instant::now();
    let encoded = encode(&input, &arena[0], &arena);
    let mut mzip_file_name = file_name.clone();
    mzip_file_name.push_str(".mzip");
    println!("compression took: {:?}", now.elapsed());
    std::fs::write(mzip_file_name, &encoded[..]).expect("problem writing");

    println!("starting decompression");
    now = Instant::now();
    let decoded = decode(&encoded, &arena[0], &arena);
    let mut decomp_file_name = file_name.clone();
    decomp_file_name.push_str(".decomp");
    println!("decompression took: {:?}", now.elapsed());
    std::fs::write(decomp_file_name, &decoded[..]).expect("problem writing");
}
