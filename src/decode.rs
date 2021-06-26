use crate::types::{ProdArena, ProdHuffman};
use std::string::String;
use std::{io, io::{Write, Read}};

const BUF_SIZE: usize = 65536;
/// out_file must be opened in append mode
pub fn decode<T,O>(in_file: &mut T, tree: &ProdHuffman, arena: &ProdArena, out_file: &mut O) -> io::Result<()> where
    T: io::Read,
    O: io::Write,
        {
    // let bits = make_bit_list(input);
    let mut buf = Box::new([0; BUF_SIZE]);
    let mut current_tree_position = tree;
    let mut bytes_read = BUF_SIZE;
    while bytes_read == BUF_SIZE {
        let mut decoded_string = String::new();
        bytes_read = in_file.by_ref().take(BUF_SIZE as u64).read(buf.as_mut())?;
        for byte_index in 0..bytes_read {
            let mut byte = buf[byte_index];
            for _i in 0..8 {
                let bit = byte % 2;
                byte = byte >> 1;
                match &current_tree_position.children {
                    Some(children) => {
                        current_tree_position = &arena[children[bit as usize]];
                        if current_tree_position.children.is_none() {
                            decoded_string.push(*current_tree_position.character.as_ref().unwrap());
                            current_tree_position = tree;
                        }
                    }
                    None => {
                        panic!("should not have iterated if didn't have child")
                    }
                }
            }
        }
        out_file.write_all(decoded_string.as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn test_decode() {
        let arena = vec![
            ProdHuffman {
                children: Some([5, 6]),
                parent: None,
                character: None,
            },
            ProdHuffman {
                children: None,
                parent: Some(5),
                character: Some('h'),
            },
            ProdHuffman {
                children: None,
                parent: Some(6),
                character: Some('l'),
            },
            ProdHuffman {
                children: None,
                parent: Some(5),
                character: Some('e'),
            },
            ProdHuffman {
                children: None,
                parent: Some(6),
                character: Some('o'),
            },
            ProdHuffman {
                children: Some([1, 3]),
                parent: Some(0),
                character: None,
            },
            ProdHuffman {
                children: Some([4, 2]),
                parent: Some(0),
                character: None,
            },
        ];
        std::fs::remove_file("test_out.txt");
        let mut tmpfile= fs::OpenOptions::new().append(true).read(true).create(true).open("test_out.txt").expect("problem creating file");
        let mut infile = fs::OpenOptions::new().write(true).read(true).open("test_in.txt").expect("test_in.txt");
        std::fs::write("test_in.txt", &[248]).unwrap();
        decode(&mut infile, &arena[0], &arena, &mut tmpfile).expect("problem deleting");
        assert_eq!(std::fs::read_to_string("test_out.txt").expect("problem deleting"), "hell".to_string());
        std::fs::remove_file("test_out.txt").expect("problem delting");
        std::fs::remove_file("test_in.txt").unwrap();
    }
}
