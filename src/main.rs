#[macro_use]
extern crate partial_application;
mod decode;
mod encode;
mod tree;
mod types;
use crate::decode::decode;
use crate::tree::{make_start_count_huffman_with_hash_map, make_tree};
use crate::types::{into_prod, Arena, Encoded, Huffman, ProdArena};
use encode::encode;
use serde_json;
use std::env;
use std::string::String;
use std::time::Instant;

// #[global_allocator]
// static ALLOCATOR: DhatAlloc = DhatAlloc;

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
    // let _dhat = Dhat::start_heap_profiling;
    println!("enter name of file:");
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    println!("file name: {:?}", file_name);
    let input = std::fs::read_to_string(&file_name).expect("something went wrong");

    println!("starting code generation");
    let mut now = Instant::now();
    let start_state = make_start_count_huffman_with_hash_map(&input);
    let huffman = start_state.0;
    let mut arena: Arena = start_state.1;
    let mut _result = make_tree(huffman, &mut arena);
    let prod_arena = into_prod(&arena);
    let duration = now.elapsed();
    pprint_huffman(&arena[huffman], &arena);
    println!("code generation took: {:?}", duration);

    println!("starting compression");
    now = Instant::now();
    let encoded = encode(input, &prod_arena[0], &prod_arena);
    let encoded_data = encoded.data;
    eprintln!("getting back into main");
    eprintln!("compression took: {:?}", now.elapsed());
    let mut mzip_file_name = file_name.clone();
    mzip_file_name.push_str(".mzip");
    let mut tree_fname = file_name.clone();
    tree_fname.push_str(".tree");
    let mut split_fname = file_name.clone();
    split_fname.push_str(".splits");
    std::fs::write(&mzip_file_name, &encoded_data[..]).expect("problem writing");
    std::fs::write(
        &split_fname,
        &serde_json::to_string(&encoded.split_locs)
            .expect("serialization error")
            .as_str(),
    )
    .expect("error writing splits");
    std::fs::write(
        &tree_fname,
        &serde_json::to_string(&prod_arena)
            .expect("serialization error")
            .as_str(),
    )
    .expect("problem writing tree");

    println!("starting decompression");
    let tree_str = std::fs::read_to_string(&tree_fname).expect("problem reading tree");
    let tree: ProdArena = serde_json::from_str(&tree_str).expect("problem deserializing");
    let compressed = std::fs::read(&mzip_file_name).expect("problem reading mzip");
    let split_locs_str = std::fs::read_to_string(&split_fname).expect("problem reading splits");
    let split_locs: Vec<usize> =
        serde_json::from_str(&split_locs_str).expect("problem deserializing");
    let to_decompress = Encoded {
        data: compressed,
        split_locs: split_locs,
    };
    now = Instant::now();
    let decoded = decode(to_decompress, &tree[0], &tree);
    let mut decomp_file_name = file_name.clone();
    decomp_file_name.push_str(".decomp");
    println!("decompression took: {:?}", now.elapsed());
    std::fs::write(decomp_file_name, &decoded[..]).expect("problem writing");
}
