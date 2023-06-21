pub fn print_bytes_vec(bytes: &Vec<u8>) {
    bytes.iter().for_each(|b| print!("{:02x} ", b));
    // println!("LAST; {}", read_i32(&bytes[0..4]));
}

pub(crate) fn read_i32(data: &[u8]) -> i32 {
    println!("len: {}", data.len());
    
    let mut bytes: [u8; 4] = [0; 4];
    bytes.copy_from_slice(&data[..4]);

    println!("len: {:?}", bytes);
    i32::from_ne_bytes(bytes)
}


