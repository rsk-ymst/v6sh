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
