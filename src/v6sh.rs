use std::{error::Error, fs::File, io::{Read, self}};
use crate::utils;
use crate::def::*;

const BLOCK_SIZE: usize = 512;
const i32_BYTES_SIZE: usize = 4;

#[derive(Debug, Clone, Default)]
struct filsys {
    s_size:  i32,
    s_fsize: i32,
    s_nfree: i32,
    // s_free:  [i32; 100],
    s_inode: i32
}

#[derive(Debug)]
pub struct bytesManager {
    pub bytes:  Vec<u8>,
    pub cursor: usize,
}

impl filsys {
    pub fn new() -> filsys {
        Default::default()
    }
}

pub fn get_block_bytes_data(file_name: &str) -> io::Result<Vec<u8>>{
    let mut all_bytes: Vec<u8> = Vec::new();

    let mut file = File::open(file_name).expect("file not found...");
    file.read_to_end(&mut all_bytes);

    utils::print_bytes_vec(&all_bytes);
    return Ok(all_bytes);
}

impl bytesManager {
    pub fn new(bytes: Vec<u8>) -> bytesManager {
        bytesManager {
            bytes:  bytes,
            cursor: 0
        }
    }

    fn forward_cursor(&mut self, count: usize) {
        #[cfg(debug_assertions)]
        println!("forward: {} --> {}", self.cursor, self.cursor + count);

        self.cursor += count;
    }

    fn backward_cursor(&mut self, count: usize) {
        #[cfg(debug_assertions)]
        println!("forward: {} --> {}", self.cursor, self.cursor + count);

        self.cursor -= count;
    }

    fn read_then_cousume_i32(&mut self) -> io::Result<i32> {
        let x = utils::read_i32(&mut self.bytes[self.cursor..self.cursor + i32_BYTES_SIZE]);
        self.forward_cursor(i32_BYTES_SIZE);

        Ok(x)
    }

    fn read_then_cousume_u32(&mut self) -> io::Result<i32> {
        let x = utils::read_i32(&mut self.bytes[self.cursor..self.cursor+4]);
        self.forward_cursor(4);

        Ok(x)
    }

    pub fn parse(&mut self) -> io::Result<usize> {
        let x = filsys::new();
        println!("{:?}", x);

        self.read_then_cousume_i32();
        self.read_then_cousume_i32();

        Ok(0)
    }

}