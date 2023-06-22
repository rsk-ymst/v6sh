use v6sh::FilSysParser;

mod shell;
mod block;
mod v6sh;
mod utils;
mod def;

static BLOCK_FILE: &'static str = "v6root";

fn main() {
    // let bm: bytesManager = v6sh::bytesManager {
    //     bytes: v6sh::get_block_bytes_data(BLOCK_FILE).unwrap(),
    //     cursor: 0,
    // };

    let bytes = v6sh::get_block_bytes_data(BLOCK_FILE).unwrap();

    let mut bm = FilSysParser::new(bytes);
    bm.parse();

}
