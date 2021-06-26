#![feature(test)]

extern crate test;

#[macro_use]
extern crate partial_application;
mod decode;
mod encode;
mod tree;
mod types;
use crate::decode::decode;
use crate::tree::{make_start_count_huffman_with_hash_map, make_tree};
use crate::types::{into_prod, Arena, Cli, ProdArena};
use encode::encode;
use serde_json;
use std::io;
use std::string::String;
use std::time::Instant;
use structopt::StructOpt;
use types::ProdHuffman;

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
    eprint!("{:?}", cli);
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
    let tree_and_data = read_compressed_files(file_name)?;
    let tree: ProdArena = tree_and_data.0;
    let mut compressed = tree_and_data.1;
    let mut decomp_file_name = file_name.clone();
    decomp_file_name.push_str(".decomp");
    match std::fs::remove_file(&decomp_file_name[..]) {
        Err(_) => (),
        _ => (),
    }
    let mut out_file = std::fs::OpenOptions::new().append(true).create(true).open(&decomp_file_name[..])?;
    decode(&mut compressed, &tree[0], &tree, &mut out_file)?;
    println!("decompression took: {:?}", now.elapsed());
    Ok(())
}

fn read_compressed_files(file_name: &String) -> io::Result<(ProdArena, std::fs::File)> {
    let mut mzip_file_name = file_name.clone();
    mzip_file_name.push_str(".mzip");
    let mut tree_fname = file_name.clone();
    tree_fname.push_str(".tree");
    let tree_str = std::fs::read_to_string(&tree_fname)?;
    let tree: ProdArena = serde_json::from_str(&tree_str).unwrap();
    let compressed = std::fs::OpenOptions::new().read(true).open(&mzip_file_name)?;
    Ok((tree, compressed))
}

fn compression_step(file_name: &String) -> std::io::Result<()> {
    let input = std::fs::read_to_string(&file_name)?;
    // parent node is always the first one in the arena
    println!("starting code generation");
    let mut now = Instant::now();
    let prod_arena: ProdArena = generate_tree(&input);
    let duration = now.elapsed();
    pprint_huffman(&prod_arena[0], &prod_arena);
    println!("code generation took: {:?}", duration);

    println!("starting compression");
    now = Instant::now();
    let mut input_file = std::fs::OpenOptions::new().read(true).open(&file_name)?;
    let mut mzip_file_name = file_name.clone();
    mzip_file_name.push_str(".mzip");
    std::fs::remove_file(&mzip_file_name);
    let mut output_file = std::fs::OpenOptions::new().create(true).append(true).open(&mzip_file_name)?;
    compress(&mut input_file, &prod_arena, &mut output_file)?;
    eprintln!("compression took: {:?}", now.elapsed());
    write_compressed_to_disk(&prod_arena, &file_name)?;
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

fn compress(input: &mut std::fs::File, arena: &ProdArena, output: &mut std::fs::File) -> std::io::Result<()> {
    let encoded = encode(input, &arena[0], &arena, output);
    encoded
}

fn write_compressed_to_disk(
    arena: &ProdArena,
    file_name: &String,
) -> std::io::Result<()> {
    let mut tree_fname = file_name.clone();
    tree_fname.push_str(".tree");
    std::fs::write(&tree_fname, &serde_json::to_string(&arena)?.as_str())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use super::*;

    #[bench]
    fn bench_decompress(b: &mut Bencher) {
        let fname = String::from("file.txt");
        b.iter(|| {
            black_box(decompression_step(&fname));
        })
    }
}
