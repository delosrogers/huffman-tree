use crate::types::{CharCount, HuffNode, Huffman};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::string::String;
use std::time::Instant;
use std::{borrow::Borrow, collections::HashMap};

pub mod types {
    use std::fmt;
    #[derive(Debug)]
    pub struct Huffman {
        pub count: i32,
        pub children: Vec<HuffNode>,
    }
    #[derive(Debug)]
    pub struct CharCount {
        pub count: i32,
        pub character: char,
    }

    impl Clone for CharCount {
        fn clone(&self) -> Self {
            CharCount {
                count: self.count,
                character: self.character,
            }
        }
    }
    #[derive(Debug)]
    pub enum HuffNode {
        Huff(Huffman),
        CharCount(CharCount),
    }

    impl Eq for HuffNode {}

    impl HuffNode {
        pub fn count(&self) -> i32 {
            match self {
                HuffNode::Huff(huffman) => huffman.count,
                HuffNode::CharCount(charcount) => charcount.count,
            }
        }
    }
    impl PartialOrd for HuffNode {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for HuffNode {
        fn eq(&self, other: &Self) -> bool {
            self.count() == other.count()
        }
    }

    impl Ord for HuffNode {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.count().cmp(&other.count())
        }
    }

    impl fmt::Display for Huffman {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.count)
        }
    }
    impl fmt::Display for HuffNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::CharCount(charcount) => write!(f, "{}", charcount),
                Self::Huff(huffman) => write!(f, "{}", huffman),
            }
        }
    }
    impl fmt::Display for CharCount {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}: {}", self.count, self.character)
        }
    }
}

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
    let input = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1000)
        .map(char::from)
        .collect();
    // let input = std::fs::read_to_string("file.txt").expect("something went wrong");
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

/// Makes a non valid Huffman tree that contains the counts of every
/// character
fn make_start_count_huffman_with_hash_map(string: &String) -> Huffman {
    let input_array = string.chars();
    let mut letters_map: HashMap<char, i32> = HashMap::new();
    for i in input_array {
        match letters_map.get(&i) {
            Some(count) => letters_map.insert(i, *count + 1),
            None => letters_map.insert(i, 1),
        };
    }
    let mut letters: Vec<HuffNode> = Vec::new();
    let mut total = 0;
    for (character, count) in letters_map.iter() {
        total += count;
        letters.push(HuffNode::CharCount(CharCount {
            count: *count,
            character: *character,
        }));
    }
    Huffman {
        count: total,
        children: letters,
    }
}

/// makes a non valid Huffman tree that contains the counts
/// of every character without using a hashmap (slow)
fn make_start_count_huffman(string: &String) -> Huffman {
    let input_array = string.chars();
    let mut letters: Vec<CharCount> = Vec::new();
    for i in input_array.clone() {
        increment_char(&mut letters, i);
    }
    let mut total: i32 = 0;
    for i in letters.clone() {
        total += i.count;
    }
    let mut huff_node_letters: Vec<HuffNode> = Vec::new();
    for i in letters {
        huff_node_letters.push(HuffNode::CharCount(i));
    }
    Huffman {
        count: total,
        children: huff_node_letters,
    }
}

///increments characters from the non make_start_count_huffman()
fn increment_char(char_count_vec: &mut Vec<CharCount>, k: char) {
    for i in 0..(char_count_vec.clone().len() as isize) {
        if char_count_vec[i as usize].character == k {
            // println!("{:?} increment char", char_count_vec[i as usize]);
            char_count_vec[i as usize].count += 1;
            return;
        }
    }
    char_count_vec.push(CharCount {
        count: 1,
        character: k,
    });
}

/// taking the non valid huffman tree that just has char counts
/// mutates it into a valid one
fn make_tree(input_tree: &mut Huffman) {
    if input_tree.children.len() > 2 {
        input_tree.children.sort();
        let smallest = input_tree.children.remove(0);
        let second_smallest = input_tree.children.remove(0);
        input_tree.children.push(HuffNode::Huff(Huffman {
            count: smallest.count() + second_smallest.count(),
            children: vec![smallest, second_smallest],
        }));
        make_tree(&mut *input_tree)
    }
}
