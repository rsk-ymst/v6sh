use crate::utils;

pub fn print_bytes_vec(bytes: &Vec<u8>) {
    bytes.iter().for_each(|b| print!("{:02x} ", b));
    // println!("LAST; {}", read_i32(&bytes[0..4]));
}

pub(crate) fn read_i32(data: &[u8]) -> i32 {
    println!("len: {}", data.len());

    let mut bytes: [u8; 4] = [0; 4];
    bytes.copy_from_slice(&data[..4]);

    #[cfg(debug_assertions)]
    println!("len: {:?}", bytes);
    
    i32::from_ne_bytes(bytes)
}

pub fn read_i16(data: &[u8]) -> i16 {
    let mut bytes: [u8; 2] = [0; 2];
    bytes.copy_from_slice(data);

    let res = i16::from_ne_bytes(bytes);

    #[cfg(debug_assertions)]
    println!("val: {}", res);

    res
}

pub fn read_u16(data: &[u8]) -> u16 {
    let mut bytes: [u8; 2] = [0; 2];
    bytes.copy_from_slice(data);

    let res = u16::from_ne_bytes(bytes);

    #[cfg(debug_assertions)]
    println!("val: {}", res);

    res
}
