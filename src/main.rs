/* 
  using original 1992 publication of md5 - RFC1321 
  (https://tools.ietf.org/html/rfc1321)
*/

/*
  ***Planning***

  1. Take in message m of arbitrary length
  
  2. Pad message with a single 1 bit and then 0's until the message length
     is congruent to 448 % 512. (len(m) - 448 % 512 = 0)
  
  3. Append a 64-bit representation of the original message length to the padded message
      - If the original message is >2^64 take the low-order 64 bits
      - Append the bits as two 32-bit words, appending the low order word first
  
  4. Build a four word buffer [A, B, C, D] where each word is a 32-bit value
  
  5. Initialize each of the buffer words to the following hex values:
      A: 01 23 45 67
      B: 89 ab cd ef
      C: fe dc ba 98
      D: 76 54 32 10
  
  6. Define four functions (x_1, x_2, x_3) => x_4 where all x's are 32-bit words:
      F(X, Y, Z) = X&Y | !X & Z
      G(X, Y, Z) = X&Z | Y & !Z
      H(X, Y, Z) = X ^ Y ^ Z
      I(X, Y, Z) = Y ^ (X | !Z)
      
      NOTE: Each of these functions should have independent tests

  7. Use a precalculated 64-element table defined by the following
      - T[i] = (4294967296 * abs(sin(i))) `div` 1 where i is in radians
  
  8. For each 16-word block in the message: 
    
        1. Set X to the next 16-word block of the message  
        2. Store the buffer words in temporary variables
            AA = A
            ...
            DD = D
        3. Run four iteration rounds as indicated in 3.4 of the RFC
        4. Increment each buffer register with it's shadow variable
            A = A + AA
            ...
            D = D + DD
        5. Repeat until entire message is digested
        
  9. Return A, B, C, D as the final message 
      - Begin with low-order byte of A and end with high-order byte of D
*/

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
