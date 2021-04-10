use crate::types::{compare_usize_pointers, Arena, Huffman};
use rayon::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, RwLock};
use std::{char, collections::HashMap};

/// Makes a non valid Huffman tree that contains the counts of every
/// character
pub fn make_start_count_huffman_with_hash_map(string: &String) -> (usize, Arena) {
    let input_array: Vec<char> = string.chars().collect();
    let mut arena: Arena = Vec::new();
    // let letters_map: RwLock<HashMap<char, AtomicI32>> = RwLock::from(HashMap::new());
    // Vec::from(input_array)
    //     .into_par_iter()
    //     .for_each(|character| {
    //         let read_map = letters_map.read().unwrap();
    //         match read_map.get(&character) {
    //             Some(count) => {
    //                 count.fetch_add(1, Ordering::Relaxed);
    //             }
    //             None => {
    //                 drop(read_map);
    //                 let mut write_map = letters_map.write().unwrap();
    //                 write_map.insert(character, AtomicI32::from(1));
    //             }
    //         }
    //     });
    let mut letters_map: HashMap<char, i32> = HashMap::new();
    for i in input_array {
        match letters_map.get_mut(&i) {
            Some(count) => *count += 1,
            None => {
                letters_map.insert(i, 1);
            }
        };
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
    // for (character, count) in letters_map.read().unwrap().iter() {
    //     let count_i32 = &count.load(Ordering::Relaxed);
    //     total += count_i32;
    //     arena.push(Huffman {
    //         count: *count_i32,
    //         children: None,
    //         parent: Some(0),
    //         character: Some(*character),
    //     });
    //     letter_idxs.push(arena.len() - 1)
    // }
    arena[0].children = Some(letter_idxs);
    arena[0].count = total;
    (0, arena)
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
