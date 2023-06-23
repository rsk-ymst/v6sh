use std::io;

use v6sh::parser::BlockDeviceParser;

mod block;
mod def;
mod shell;
mod utils;
mod v6sh;

static BLOCK_FILE: &'static str = "v6root";

fn main() {

    let bytes = v6sh::get_block_bytes_data(BLOCK_FILE).unwrap();

    let mut bm = BlockDeviceParser::new(bytes);
    let nodes = bm.parse().unwrap();
    println!("{}", nodes.len());
    println!("------------------- hello, v6sh");
    print!(">> ");
    print!(" ");

    let mut current_node = nodes[1].clone();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let args: Vec<&str> = input.split_whitespace().collect();

        match args[0] {
            "ls" => {
                let _ = &current_node.ls();
            },
            "cd" => {
                let id = current_node.get_nodeId_from_table(args[1].to_string());
                println!("move --> {}", id);
                current_node = nodes[id as usize].clone();
            },
            _ => {

            }
        }
    }
}
