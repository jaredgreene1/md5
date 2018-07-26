fn F(X: u8, Y: u8, Z: u8) -> u8 {
    X & Y | !X & Z
}

fn G(X: u8, Y: u8, Z: u8) -> u8 {
    X & Z | Y & !Z
}

fn H(X: u8, Y: u8, Z: u8) -> u8 {
    X ^ Y ^ Z
}

fn I(X: u8, Y: u8, Z: u8) -> u8 {
    Y ^ (X | !Z)
}

fn T(i: f32) -> f32 {
    let INT_CONST = 4_294_967_296f32;
    INT_CONST * i.sin().abs()
}


fn _rnd_1(a: u8, b: u8, c: u8, d: u8, X: [u8; 16], n0: i32, n1: i32, n2: i32, func) {
    a = b + ((a + func(b, c, d) + X[n0] + T(n2)) << n1);
}


pub fn rnd_1(A: u8, B: u8, C: u8, D: u8) {
    let func = F;
    /*
    [ABCD  0  7  1]  [DABC  1 12  2]  [CDAB  2 17  3]  [BCDA  3 22  4]
    [ABCD  4  7  5]  [DABC  5 12  6]  [CDAB  6 17  7]  [BCDA  7 22  8]
    [ABCD  8  7  9]  [DABC  9 12 10]  [CDAB 10 17 11]  [BCDA 11 22 12]
    [ABCD 12  7 13]  [DABC 13 12 14]  [CDAB 14 17 15]  [BCDA 15 22 16]
    */
}

pub fn rnd_2(A: u8, B: u8, C: u8, D: u8) {
    let func = G;
    /*
    [ABCD  1  5 17]  [DABC  6  9 18]  [CDAB 11 14 19]  [BCDA  0 20 20]
    [ABCD  5  5 21]  [DABC 10  9 22]  [CDAB 15 14 23]  [BCDA  4 20 24]
    [ABCD  9  5 25]  [DABC 14  9 26]  [CDAB  3 14 27]  [BCDA  8 20 28]
    [ABCD 13  5 29]  [DABC  2  9 30]  [CDAB  7 14 31]  [BCDA 12 20 32]
    */
}

pub fn rnd_3(A: u8, B: u8, C: u8, D: u8) {
    let func = H;
    /*
    [ABCD  5  4 33]  [DABC  8 11 34]  [CDAB 11 16 35]  [BCDA 14 23 36]
    [ABCD  1  4 37]  [DABC  4 11 38]  [CDAB  7 16 39]  [BCDA 10 23 40]
    [ABCD 13  4 41]  [DABC  0 11 42]  [CDAB  3 16 43]  [BCDA  6 23 44]
    [ABCD  9  4 45]  [DABC 12 11 46]  [CDAB 15 16 47]  [BCDA  2 23 48]
    */
}

pub fn rnd_4(A: u8, B: u8, C: u8, D: u8) {
    let func = I;
    /*
    [ABCD  0  6 49]  [DABC  7 10 50]  [CDAB 14 15 51]  [BCDA  5 21 52]
    [ABCD 12  6 53]  [DABC  3 10 54]  [CDAB 10 15 55]  [BCDA  1 21 56]
    [ABCD  8  6 57]  [DABC 15 10 58]  [CDAB  6 15 59]  [BCDA 13 21 60]
    [ABCD  4  6 61]  [DABC 11 10 62]  [CDAB  2 15 63]  [BCDA  9 21 64]
    */
}
