use std::string::String;

// struct letters {
//     a: i32,
//     b: i32,
//     c: i32,
//     d: i32,
//     e: i32,
//     f: i32,
//     g: i32,
//     h: i32,
//     i: i32,
//     j: i32,
//     k: i32,
//     l: i32,
//     m: i32,
//     n: i32,
//     o: i32,
//     p: i32,
//     q: i32,
//     r: i32,
//     s: i32,
//     t: i32,
//     u: i32,
//     v: i32,
//     w: i32,
//     x: i32,
//     y: i32,
//     z: i32,
// }

fn main() {
    let input = "mattias".to_string();
    let mut huffman = make_start_count_huffman(&input);
    println!("{:?}", huffman);
    let result = make_tree(&mut huffman);
    println!("{:?}", *result);
}

fn make_start_count_huffman(string: &String) -> Huffman {
    let input_array = string.chars();
    let mut letters: Vec<CharCount> = Vec::new();
    for i in input_array.clone() {
        println!("{:?}", i);
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
fn increment_char(char_count_vec: &mut Vec<CharCount>, k: char) -> &mut Vec<CharCount> {
    for i in 0..(char_count_vec.clone().len() as isize) {
        // println!(
        //     "{:?} increment char outside of if, k: {:?}",
        //     char_count_vec[i as usize], k
        // );
        if char_count_vec[i as usize].character == k {
            // println!("{:?} increment char", char_count_vec[i as usize]);
            char_count_vec[i as usize].count += 1;
            return &mut *char_count_vec;
        }
    }
    char_count_vec.push(CharCount {
        count: 1,
        character: k,
    });
    &mut *char_count_vec
}
#[derive(Debug)]
struct Huffman {
    count: i32,
    children: Vec<HuffNode>,
}
#[derive(Debug)]
struct CharCount {
    count: i32,
    character: char,
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
enum HuffNode {
    Huff(Huffman),
    CharCount(CharCount),
}

impl Eq for HuffNode {}

impl HuffNode {
    fn count(&self) -> i32 {
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

fn make_tree(input_tree: &mut Huffman) -> &Huffman {
    if input_tree.children.len() > 2 {
        input_tree.children.sort();
        let smallest = input_tree.children.remove(0);
        let second_smallest = input_tree.children.remove(0);
        input_tree.children[0] = HuffNode::Huff(Huffman {
            count: smallest.count() + second_smallest.count(),
            children: vec![smallest, second_smallest],
        });
        make_tree(&mut *input_tree)
    } else {
        input_tree
    }
}
