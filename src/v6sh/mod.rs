use crate::def::*;
use crate::utils;
use std::{
    error::Error,
    fs::File,
    io::{self, Read},
};

pub mod inode;
pub mod parser;

use self::inode::Inode;

const BLOCK_SIZE: usize = 512;
const i32_BYTES_SIZE: usize = 4;
const i16_BYTES_SIZE: usize = 2;
const u16_BYTES_SIZE: usize = 2;

const DirSize: usize = 16;

#[derive(Debug, Clone)]
pub struct filSys {
    s_isize: i16,
    s_fsize: i16,
    s_nfree: i16,
    s_free: [i16; 100],
    s_ninode: i16,
    s_inode: [i16; 100],
    s_flock: u8,
    s_ilock: u8,
    s_fmod: u8,
    s_ronly: u8,
    s_time: [i16; 2],
    s_pad: [i16; 50],
}


impl filSys {
    pub fn new() -> filSys {
        filSys {
            s_isize: 0,
            s_fsize: 0,
            s_nfree: 0,
            s_free: [0; 100],
            s_ninode: 0,
            s_inode: [0; 100],
            s_flock: 0,
            s_ilock: 0,
            s_fmod: 0,
            s_ronly: 0,
            s_time: [0; 2],
            s_pad: [0; 50],
        }
    }
}

pub fn get_block_bytes_data(file_name: &str) -> io::Result<Vec<u8>> {
    let mut all_bytes: Vec<u8> = Vec::new();

    let mut file = File::open(file_name).expect("file not found...");
    file.read_to_end(&mut all_bytes);

    // utils::print_bytes_vec(&all_bytes);
    return Ok(all_bytes);
}
