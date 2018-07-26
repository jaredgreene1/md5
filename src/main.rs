mod digest;

fn pad_req(buf_size: usize) -> u8 {
    // Calulate required padding for congruence with 448 mod 512
    let bits_448: i32 = 56;
    let bits_512: i32 = 64;
    let buf_size = buf_size as i32; // cast to allow subtraction with i32
    ((bits_448 - buf_size).abs() % bits_512) as u8
}

#[test]
fn test_pad_req() {
    let cases = [(8, 48), (90012, 36), (3, 53)];
    for (ln, pad) in cases.iter() {
        assert_eq!(pad_req(*ln), *pad);
    }
}

fn pad_rnd1(msg: &[u8]) -> Vec<u8> {
    // pad until len(msg) is congruent with 448 mod 512
    let pad_1 = 0b10000000;
    let pad_0 = 0b00000000;

    let mut buf: Vec<u8> = Vec::new();
    for byte in msg {
        buf.push(*byte);
    }
    buf.push(pad_1);
    for i in 0..pad_req(buf.len()) {
        buf.push(pad_0);
    }

    buf
}

fn pad_rnd2(mut msg: Vec<u8>) -> Vec<u8> {
    // Pad the now congruent message with the message length
    let msg_len: u64 = msg.len() as u64;
    for i in 0..4 {
        let msg_byte = msg_len >> (8 * i);
        msg.push(msg_byte as u8);
    }
    msg
}

fn digest(msg: Vec<u8>) -> () {
    let a: [u8; 4] = [0x01, 0x23, 0x45, 0x67];
    let b: [u8; 4] = [0x89, 0xab, 0xcd, 0xef];
    let c: [u8; 4] = [0xfe, 0xdc, 0xba, 0x98];
    let d: [u8; 4] = [0x76, 0x54, 0x32, 0x10];
}

// digestion functions
/*
    fn f() {}
    fn g() {}
    fn h() {}
    fn i() {}
*/

fn main() {
    let mut message = String::new();
    println!("Please input message to hash: ");
    std::io::stdin()
        .read_line(&mut message)
        .expect("Message input failure");

    let m_buf: &[u8] = message.as_bytes();
    let p1_buf: Vec<u8> = pad_rnd1(m_buf);
    let p2_buf: Vec<u8> = pad_rnd2(p1_buf);
    digest(p2_buf);
}
