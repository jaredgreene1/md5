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






fn main() {
    println!("Hello, world!");
}
