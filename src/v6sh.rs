use std::{error::Error, fs::File, io::{Read, self}};
use crate::utils;
use crate::def::*;

const BLOCK_SIZE: usize     = 512;
const i32_BYTES_SIZE: usize = 4;
const i16_BYTES_SIZE: usize = 2;

#[derive(Debug, Clone)]
pub struct filSys {
    s_size:   i16,
    s_fsize:  i16,
    s_nfree:  i16,
    s_free:   [i16; 100],
    s_ninode: i16,
    s_inode:  [i16; 100],
    s_flock:  u8,
    s_ilock:  u8,
    s_fmod:   u8,
    s_ronly:  u8,
    s_time:   [i16; 2],
    s_pad:    [i16; 50],
}

#[derive(Debug)]
pub struct FilSysParser {
    pub bytes:  Vec<u8>,
    pub cursor: usize,
    pub filSys: filSys,
}

impl filSys {
    pub fn new() -> filSys {
        filSys {
            s_size: 0,
            s_fsize: 0,
            s_nfree: 0,
            s_free: [0; 100],
            s_ninode: 0,
            s_inode: [0; 100],
            s_flock: 0,
            s_ilock: 0,
            s_fmod: 0,
            s_ronly: 0,
            s_time:  [0; 2],
            s_pad:  [0; 50],
        }
    }
}

pub fn get_block_bytes_data(file_name: &str) -> io::Result<Vec<u8>>{
    let mut all_bytes: Vec<u8> = Vec::new();

    let mut file = File::open(file_name).expect("file not found...");
    file.read_to_end(&mut all_bytes);

    // utils::print_bytes_vec(&all_bytes);
    return Ok(all_bytes);
}

impl FilSysParser {
    pub fn new(bytes: Vec<u8>) -> FilSysParser {
        FilSysParser {
            bytes:  bytes,
            cursor: 0,
            filSys: filSys::new()
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

    /* 与えらえたbit数消費する関数 */
    fn consume_bytes(&mut self, size: usize) -> io::Result<()> {
        self.forward_cursor(size);

        Ok(())
    }

    fn read_then_consume_i32(&mut self) -> io::Result<i32> {
        let x = utils::read_i32(&mut self.bytes[self.cursor..self.cursor + i32_BYTES_SIZE]);
        self.forward_cursor(i32_BYTES_SIZE);

        Ok(x)
    }

    fn read_then_consume_i16(&mut self) -> io::Result<i16> {
        let x = utils::read_i16(&mut self.bytes[self.cursor..self.cursor + i16_BYTES_SIZE]);
        self.forward_cursor(i16_BYTES_SIZE);

        Ok(x)
    }

    // fn read_then_consume_u32(&mut self) -> io::Result<i32> {
    //     let x = utils::read_i32(&mut self.bytes[self.cursor..self.cursor+4]);
    //     self.forward_cursor(4);

    //     Ok(x)
    // }

    // fn read_then_consume_i16_array(&mut self, size: usize) -> io::Result<&[i16]> {
    //     let mut buffer = &[0; 100];

    //     for i in 0..size {
    //         buffer[i] = utils::read_i16(&mut self.bytes[self.cursor..self.cursor + i16_BYTES_SIZE]);
    //         self.forward_cursor(i16_BYTES_SIZE);
    //     }

    //     Ok(buffer)
    // }

    pub fn parse(&mut self) -> io::Result<usize> {

        self.consume_bytes(BLOCK_SIZE)?; // 起動用領域分の消費

        self.filSys.s_size  = self.read_then_consume_i16().unwrap();
        self.filSys.s_fsize = self.read_then_consume_i16().unwrap();
        self.filSys.s_nfree = self.read_then_consume_i16().unwrap();

        for i in 0..100 {
            self.filSys.s_free[i as usize] = self.read_then_consume_i16().unwrap();
        }

        self.filSys.s_ninode = self.read_then_consume_i16().unwrap();

        for i in 0..100 {
            self.filSys.s_inode[i as usize] = self.read_then_consume_i16().unwrap();
        }

        self.filSys.s_ninode = self.read_then_consume_i16().unwrap();

        println!("{:?}", self.filSys.s_free);


        Ok(0)
    }

}
