use std::io;

use crate::utils;

use super::{filSys, inode::Inode};

const BLOCK_SIZE: usize = 512;
const i32_BYTES_SIZE: usize = 4;
const i16_BYTES_SIZE: usize = 2;
const u16_BYTES_SIZE: usize = 2;

const DirSize: usize = 16;



#[derive(Debug)]
pub struct BlockDeviceParser {
    pub bytes: Vec<u8>,
    pub cursor: usize,
    pub filSys: filSys,
    inodes: Vec<Inode>,
}

impl BlockDeviceParser {
    pub fn new(bytes: Vec<u8>) -> BlockDeviceParser {
        BlockDeviceParser {
            bytes: bytes,
            cursor: 0,
            filSys: filSys::new(),
            inodes: Vec::new(),
        }
    }

    fn forward_cursor(&mut self, count: usize) {
        #[cfg(debug_assertions)]
        println!("forward: {} --> {}", self.cursor, self.cursor + count);

        #[cfg(debug_assertions)]
        println!("now: {:02x} {:02x}", self.bytes[self.cursor], self.bytes[self.cursor + 1]);

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
        #[cfg(debug_assertions)]
        println!("***************************");

        let x = utils::read_i16(&mut self.bytes[self.cursor..self.cursor + i16_BYTES_SIZE]);
        self.forward_cursor(i16_BYTES_SIZE);

        Ok(x)
    }

    fn read_then_consume_u16(&mut self) -> io::Result<u16> {
        #[cfg(debug_assertions)]
        println!("***************************");

        let x = utils::read_u16(&mut self.bytes[self.cursor..self.cursor + u16_BYTES_SIZE]);
        self.forward_cursor(u16_BYTES_SIZE);

        Ok(x)
    }

    fn read_then_consume_u8(&mut self) -> io::Result<u8> {
        let x = self.bytes[self.cursor];
        self.forward_cursor(1);

        Ok(x)
    }

    /* 解析のエントリポイントであり、メインストリーㇺ */
    pub fn parse(&mut self) -> io::Result<Vec<Inode>> {
        self.consume_bytes(BLOCK_SIZE)?; // 起動用領域分の消費
        self.parse_super_block();

        // self.dbg_print("s_isize");
        // self.filSys.s_isize = self.read_then_consume_i16().unwrap();

        // self.dbg_print("s_fsize");
        // self.filSys.s_fsize = self.read_then_consume_i16().unwrap();

        // self.dbg_print("free");
        // self.filSys.s_nfree = self.read_then_consume_i16().unwrap();
        // for i in 0..100 {
        //     self.filSys.s_free[i as usize] = self.read_then_consume_i16().unwrap();
        // }

        // self.dbg_print("node");
        // self.filSys.s_ninode = self.read_then_consume_i16().unwrap();
        // for i in 0..100 {
        //     self.filSys.s_inode[i as usize] = self.read_then_consume_i16().unwrap();
        // }

        // /* char zone -------------------------------------------------- */
        // self.dbg_print("flock");
        // self.filSys.s_flock = self.read_then_consume_u8().unwrap();
        // self.dbg_print("ilock");
        // self.filSys.s_ilock = self.read_then_consume_u8().unwrap();
        // self.dbg_print("fmod");
        // self.filSys.s_fmod = self.read_then_consume_u8().unwrap();
        // self.dbg_print("ronly");
        // self.filSys.s_ronly = self.read_then_consume_u8().unwrap();

        // /* last zone -------------------------------------------------- */
        // self.dbg_print("s_time");
        // for i in 0..2 {
        //     self.filSys.s_time[i as usize] = self.read_then_consume_i16().unwrap();
        // }

        // self.dbg_print("node");
        // for i in 0..50 {
        //     self.filSys.s_pad[i as usize] = self.read_then_consume_i16().unwrap();
        // }

        // #[cfg(debug_assertions)]
        // println!("{:#?}", self.filSys);

        self.parse_inode_block();
        // self.createIdexTransact();
        for (i, node) in self.inodes.clone().iter().enumerate() {
            if node.metadata.is_dir {
                let x = self.create_idx(node.clone());
                self.inodes[i] = x;
            }
        }

        // #[cfg(debug_assertions)]
        for (i, node) in self.inodes.iter().enumerate() {
            if node.metadata.keys.len() > 0 {
                println!("{i}: {:#?}", node.metadata.keys);
            }
        }

        // println!("{:#?}", self.inodes);
        println!("end");

        Ok(self.inodes.clone())
    }

    pub fn search_next_zero(&mut self) -> usize {
        let mut cur = self.cursor;

        while cur <= self.bytes.len() {
            if self.bytes[cur] == 0 {
                return cur;
            }
            cur += 1;
        }

        0
    }

    // fn createIdx(&mut self) -> usize {

    // }

    pub fn parse_super_block(&mut self) {
        self.dbg_print("s_isize");
        self.filSys.s_isize = self.read_then_consume_i16().unwrap();

        self.dbg_print("s_fsize");
        self.filSys.s_fsize = self.read_then_consume_i16().unwrap();

        self.dbg_print("free");
        self.filSys.s_nfree = self.read_then_consume_i16().unwrap();
        for i in 0..100 {
            self.filSys.s_free[i as usize] = self.read_then_consume_i16().unwrap();
        }

        self.dbg_print("node");
        self.filSys.s_ninode = self.read_then_consume_i16().unwrap();
        for i in 0..100 {
            self.filSys.s_inode[i as usize] = self.read_then_consume_i16().unwrap();
        }

        /* char zone -------------------------------------------------- */
        self.dbg_print("flock");
        self.filSys.s_flock = self.read_then_consume_u8().unwrap();
        self.dbg_print("ilock");
        self.filSys.s_ilock = self.read_then_consume_u8().unwrap();
        self.dbg_print("fmod");
        self.filSys.s_fmod = self.read_then_consume_u8().unwrap();
        self.dbg_print("ronly");
        self.filSys.s_ronly = self.read_then_consume_u8().unwrap();

        /* last zone -------------------------------------------------- */
        self.dbg_print("s_time");
        for i in 0..2 {
            self.filSys.s_time[i as usize] = self.read_then_consume_i16().unwrap();
        }

        self.dbg_print("node");
        for i in 0..50 {
            self.filSys.s_pad[i as usize] = self.read_then_consume_i16().unwrap();
        }

        #[cfg(debug_assertions)]
        println!("{:#?}", self.filSys);
    }

    pub fn parse_inode_block(&mut self) -> i16 {
        // 1ブロックに何個inode入っているか？ ---> 512/32 = 16,
        let max_inode_number = BLOCK_SIZE / 32 * self.filSys.s_isize as usize;

        self.inodes = vec![Inode::new(); max_inode_number];
        self.cursor = 2 * BLOCK_SIZE;

        /* 配列の添字とInode番号を合わせるため、[0]に空データを入れる  */
        self.inodes.push(Inode::new());
        for i in 1..=max_inode_number {
            self.inodes[i as usize] = self.parse_single_inode();
        }

        #[cfg(debug_assertions)]
        println!("len: {:#?}", self.inodes.len());

        0
    }

    // pub fn createIdexTransact(&mut self) {
    //     for (_, node) in self.inodes.iter_mut().enumerate() {
    //         if node.metadata.is_dir {
    //             let x = *self.create_idx(node.clone());
    //             *node = x;
    //         }
    //     }
    // }

    pub fn create_idx(&mut self, mut node: Inode) -> Inode {
        // for (i, node) in self.inodes.clone().iter_mut().enumerate() {
            for i in 0..8 {
                let offset = (node.i_addr[i] as usize) * BLOCK_SIZE;
                let mut is_leak = false;

                for j in 0..(BLOCK_SIZE / DirSize) {
                    // self.forward_cursor(j * DirSize + offset);

                    if j >= ((node.metadata.size as usize - i * (BLOCK_SIZE)) / DirSize) {
                        is_leak = true;
                        break;
                    }

                    self.cursor = j * DirSize + offset;
                    let num = self.read_then_consume_u16().unwrap();

                    let entry_point = self.cursor;
                    let end_point = self.search_next_zero();

                    if end_point - entry_point <= 0 {
                        is_leak = true;
                        break;
                    }

                    #[cfg(debug_assertions)]
                    println!("start: {entry_point}, end: {end_point}");

                    let name = std::str::from_utf8(&self.bytes[entry_point..end_point])
                        .unwrap_or("error")
                        .to_string();
                    println!("**********\nextract name: {}", name);

                    if name == "error" {
                        is_leak = true;
                        break;
                    }

                    node.metadata.fTable.insert(name.clone(), num.into());
                    node.metadata.keys.push(name.clone());

                    // self.inodes[i] = node.clone();
                }

                if is_leak {
                    break;
                }

                self.cursor = offset;
            }
        // }

        // #[cfg(debug_assertions)]
        // println!("IDX_END-----------------------");

        node.clone()
    }

    pub fn parse_single_inode(&mut self) -> Inode {
        let mut node = Inode::new();

        node.i_mode = self.read_then_consume_i16().unwrap();
        node.i_nlink = self.read_then_consume_u8().unwrap();
        node.i_uid = self.read_then_consume_u8().unwrap();
        node.i_gid = self.read_then_consume_u8().unwrap();
        node.i_size0 = self.read_then_consume_u8().unwrap() as i8;
        node.i_size1 = self.read_then_consume_u16().unwrap();

        for i in 0..8 {
            node.i_addr[i] = self.read_then_consume_i16().unwrap();
        }
        for i in 0..2 {
            node.i_atime[i] = self.read_then_consume_i16().unwrap();
        }
        for i in 0..2 {
            node.i_mtime[i] = self.read_then_consume_i16().unwrap();
        }

        node.metadata.size = (node.i_size0 as u32 & 0xff) << 16 | node.i_size1 as u32;
        node.metadata.is_dir = ((node.i_mode >> 14) & 1 == 1) && (node.i_mode >> 13 & 1) != 1;
        node.metadata.is_large = ((node.i_mode >> 12) & 1) == 1;

        #[cfg(debug_assertions)]
        println!("{:#?}", node);

        node
    }

    pub fn dbg_print(&self, str: &str) {
        #[cfg(debug_assertions)]
        println!("--------------------------------- {}", str);
    }
}
