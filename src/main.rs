use std::io;

use v6sh::parser::BlockDeviceParser;
mod v6sh;

static BLOCK_FILE: &'static str = "v6root";

fn main() {

    let bytes = v6sh::get_block_bytes_data(BLOCK_FILE).unwrap();

    let mut bm = BlockDeviceParser::new(bytes);
    let nodes = bm.parse().unwrap();

    println!("inodes len: {}", nodes.len());
    println!("------------------- hello, v6sh");

    let mut current_node = nodes[1].clone();

    loop {
        println!(">> ");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let args: Vec<&str> = input.split_whitespace().collect();

        match args[0] {
            "ls" => {
                if args.len() == 1 {
                    let _ = &current_node.ls();
                } else {
                    match args[1] {
                        "-l" => {
                            for (_, name) in current_node.metadata.keys.iter().enumerate() {
                                let id = current_node.get_nodeId_from_table(name.to_string());
                                let child_node = nodes[id as usize].clone();

                                child_node.parse_permission_info(name.to_string());
                            }
                        }
                        _ => {panic!("invalid option");}
                    }
                }
            },

            "cd" => {
                let id = current_node.get_nodeId_from_table(args[1].to_string());
                println!("move --> {}", id);
                current_node = nodes[id as usize].clone();
            },

            _ => {
                panic!("invalid command");
            }
        }
    }
}
