pub fn u8x4_to_u32(u8s: [u8; 4]) -> u32 {
    let p1 = (u8s[0] as u32) << 0;
    let p2 = (u8s[1] as u32) << 8;
    let p3 = (u8s[2] as u32) << 16;
    let p4 = (u8s[3] as u32) << 24;

    let result = p1 | p2 | p3 | p4;
    result
}

pub fn u8ref_to_u32(u8s: &[u8]) -> u32 {
    let p1 = (u8s[0] as u32) << 0;
    let p2 = (u8s[1] as u32) << 8;
    let p3 = (u8s[2] as u32) << 16;
    let p4 = (u8s[3] as u32) << 24;

    let result = p1 | p2 | p3 | p4;
    result
}