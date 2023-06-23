use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
pub struct Inode {
    pub i_mode: i16,
    pub i_nlink: u8,
    pub i_uid: u8,
    pub i_gid: u8,
    pub i_size0: i8,
    pub i_size1: u16,
    pub i_addr: [i16; 8],
    pub i_atime: [i16; 2],
    pub i_mtime: [i16; 2],
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub is_dir: bool,
    pub is_large: bool,
    pub size: u32,
    pub keys: Vec<String>,
    pub fTable: HashMap<String, i32>,
}

impl Inode {
    pub fn new() -> Self {
        Inode {
            i_mode: 0,
            i_nlink: 0,
            i_uid: 0,
            i_gid: 0,
            i_size0: 0,
            i_size1: 0,
            i_addr: [0; 8],
            i_atime: [0; 2],
            i_mtime: [0; 2],
            metadata: Metadata {
                is_dir: false,
                is_large: false,
                size: 0,
                keys: Vec::new(),
                fTable: HashMap::new(),
            },
        }
    }

    pub fn ls(&self) {
        let mut content = String::new();
        let joined_lines = self.metadata.keys.join("\n");
        // let x = format!("{:#?}", self.metadata.keys);

        println!("{joined_lines}");
    }

    // pub fn ls_l(&self) {
    //     let mut content = String::new();
    //     let joined_lines = self.metadata.keys.join("\n");

    //     for (i, name) in self.metadata.keys.iter().enumerate() {
    //         let x = self.get_nodeId_from_table(name);
    //     }

    //     // let x = format!("{:#?}", self.metadata.keys);

    //     println!("{joined_lines}");
    // }

    // pub fn cd(&self, path: &str) {
    //     self.
    // }

    pub fn get_nodeId_from_table(&self, name: String) -> i32 {
        *self.metadata.fTable.get(name.as_str()).unwrap()
    }

    pub fn parse_permission_info(&self, name: String) {
        let is_dir = if self.metadata.is_dir {"d"} else {"-"};

        let user_r = if self.i_mode & 0b_0000_0000_0010_0000 != 0 {"r"} else {"-"};
        let user_w = if self.i_mode & 0b_0000_0000_0100_0000 != 0 {"w"} else {"-"};
        let user_x = if self.i_mode & 0b_0000_0000_1000_0000 != 0 {"x"} else {"-"};
        // let user_r = if self.i_mode & 1 != 1 {"r"} else {"-"};
        // let user_w = if self.i_mode & 2 != 1 {"w"} else {"-"};
        // let user_x = if self.i_mode & 4 != 1 {"x"} else {"-"};

        let group_r = if self.i_mode & 0b_0000_0000_0000_1000 != 0 {"r"} else {"-"};
        let group_w = if self.i_mode & 0b_0000_0000_0001_0000 != 0 {"w"} else {"-"};
        let group_x = if self.i_mode & 0b_0000_0000_0010_0000 != 0 {"x"} else {"-"};

        let other_r = if self.i_mode & 0b_0000_0000_0000_0001 != 0 {"r"} else {"-"};
        let other_w = if self.i_mode & 0b_0000_0000_0000_0010 != 0 {"w"} else {"-"};
        let other_x = if self.i_mode & 0b_0000_0000_0000_0100 != 0 {"x"} else {"-"};


        let x = format!("{is_dir}{user_r}{user_w}{user_x}{group_r}{group_w}{group_x}{other_r}{other_w}{other_x} {:>8} {:>8}", self.metadata.size, name);
        println!("{x}");
    }

    // pub fn parseInodeBlock()
}
