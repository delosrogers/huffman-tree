#[macro_use]
extern crate partial_application;
mod decode;
mod encode;
mod tree;
mod types;
use crate::decode::decode;
use crate::tree::{make_start_count_huffman_with_hash_map, make_tree};
use crate::types::{into_prod, Arena, Cli, Huffman, ProdArena};
use encode::encode;
use serde_json;
use std::env;
use std::io;
use std::string::String;
use std::time::Instant;
use structopt::StructOpt;
use types::ProdHuffman;

// #[global_allocator]
// static ALLOCATOR: DhatAlloc = DhatAlloc;

fn pprint_huffman(tree: &ProdHuffman, arena: &ProdArena) {
    fn _pprint_huffman(node: &ProdHuffman, prefix: String, last: bool, arena: &ProdArena) {
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
    let cli = Cli::from_args();
    let file_name = &cli.file_name;
    eprintln!("file name: {:?}", file_name);
    if cli.decompress_option == "1" {
        eprintln!("starting decompression");
        decompression_step(file_name).expect("error decompressing")
    } else {
        compression_step(file_name).expect("error compressing");
    }
}

fn decompression_step(file_name: &String) -> io::Result<()> {
    let now = Instant::now();
    let mut tree: ProdArena = Vec::new();
    let mut compressed = vec![];
    match read_compressed_files(file_name)? {
        (tr, cmp) => {
            tree = tr;
            compressed = cmp;
        }
    }
    let decoded = decode(&compressed, &tree[0], &tree);
    let mut decomp_file_name = file_name.clone();
    decomp_file_name.push_str(".decomp");
    println!("decompression took: {:?}", now.elapsed());
    std::fs::write(decomp_file_name, &decoded[..])?;
    Ok(())
}

fn read_compressed_files(file_name: &String) -> io::Result<(ProdArena, Vec<u8>)> {
    let mut mzip_file_name = file_name.clone();
    mzip_file_name.push_str(".mzip");
    let mut tree_fname = file_name.clone();
    tree_fname.push_str(".tree");
    let tree_str = std::fs::read_to_string(&tree_fname)?;
    let tree: ProdArena = serde_json::from_str(&tree_str).unwrap();
    let compressed = std::fs::read(&mzip_file_name)?;
    Ok((tree, compressed))
}

fn compression_step(file_name: &String) -> std::io::Result<()> {
    let input = std::fs::read_to_string(&file_name)?;
    // parent node is always the first one in the arena
    println!("starting code generation");
    let mut now = Instant::now();
    let mut prod_arena: ProdArena = generate_tree(&input);
    let duration = now.elapsed();
    pprint_huffman(&prod_arena[0], &prod_arena);
    println!("code generation took: {:?}", duration);

    println!("starting compression");
    now = Instant::now();
    let compressed = compress(input, &prod_arena);
    eprintln!("compression took: {:?}", now.elapsed());
    write_compressed_to_disk(&compressed, &prod_arena, &file_name)?;
    Ok(())
}

fn generate_tree(input: &String) -> ProdArena {
    let start_state = make_start_count_huffman_with_hash_map(&input);
    let huffman = start_state.0;
    let mut arena: Arena = start_state.1;
    let mut _result = make_tree(huffman, &mut arena);
    let prod_arena = into_prod(&arena);
    prod_arena
}

fn compress(input: String, arena: &ProdArena) -> Vec<u8> {
    let encoded = encode(input, &arena[0], &arena);
    encoded
}

fn write_compressed_to_disk(
    compressed: &Vec<u8>,
    arena: &ProdArena,
    file_name: &String,
) -> std::io::Result<()> {
    let mut mzip_file_name = file_name.clone();
    mzip_file_name.push_str(".mzip");
    let mut tree_fname = file_name.clone();
    tree_fname.push_str(".tree");
    std::fs::write(&mzip_file_name, &compressed[..])?;
    std::fs::write(&tree_fname, &serde_json::to_string(&arena)?.as_str())?;
    Ok(())
}
