use crate::types::{compare_usize_pointers, Arena, Huffman};
use std::{char, collections::HashMap};
use std::io::{Read};
use super::BUF_SIZE;

/// Makes a flat Huffman tree that contains the counts of every
/// character but needs to structred by count
/// returns arena ref to root of tree and the arena
pub fn make_start_count_huffman_with_hash_map<T: Read>(in_file: &mut T) -> std::io::Result<(usize, Arena)> {
    let mut buf: Box<[u8]> = Box::new([0; BUF_SIZE]);
    let mut arena: Arena = Vec::new();
    let mut letters_map: HashMap<char, i32> = HashMap::new();
    let mut bytes_read = BUF_SIZE;
    while bytes_read == BUF_SIZE {
        bytes_read = in_file.by_ref().take(BUF_SIZE as u64).read(buf.as_mut())?;
        // TODO make this work with unicode
        let input_array: Vec<char> = std::str::from_utf8(&buf[0..bytes_read]).unwrap().chars().collect();
        for i in input_array {
            match letters_map.get_mut(&i) {
                Some(count) => *count += 1,
                None => {
                    letters_map.insert(i, 1);
                }
            };
        }
    }
    let mut letter_idxs = Vec::new();
    let mut total = 0;
    let huffman = Huffman {
        count: 0,
        children: None,
        parent: None,
        character: None,
    };
    arena.push(huffman);
    for (character, count) in letters_map.iter() {
        total += count;
        arena.push(Huffman {
            count: *count,
            children: None,
            parent: Some(0),
            character: Some(*character),
        });
        letter_idxs.push(arena.len() - 1)
    }
    arena[0].children = Some(letter_idxs);
    arena[0].count = total;
    Ok((0, arena))
}

pub fn make_tree(huffman_arena_ref: usize, arena: &mut Arena) {
    if arena[huffman_arena_ref].children.clone().unwrap().len() > 2 {
        let mut new_children = arena[huffman_arena_ref].children.clone().unwrap();
        new_children.sort_by(partial!(compare_usize_pointers => _, _, arena));
        let mut parent = Huffman {
            count: 0,
            children: None,
            parent: Some(huffman_arena_ref),
            character: None,
        };
        let smallest_arena_ref: usize = new_children.remove(0);
        let second_smallest_arena_ref: usize = new_children.remove(0);
        arena[smallest_arena_ref].parent = Some(arena.len());
        arena[second_smallest_arena_ref].parent = Some(arena.len());
        parent.count = arena[smallest_arena_ref].count + arena[second_smallest_arena_ref].count;
        parent.children = Some(vec![smallest_arena_ref, second_smallest_arena_ref]);
        arena.push(parent);
        new_children.push(arena.len() - 1);
        arena[huffman_arena_ref].children = Some(new_children);
        make_tree(huffman_arena_ref, arena)
    }
}
