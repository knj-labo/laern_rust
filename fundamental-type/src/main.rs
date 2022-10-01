fn build_vector()->Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

fn main() {
    assert_eq!( 10_i8 as u16, 10_u16);
    assert_eq!( 2525_u16 as i16, 2525_i16);
    assert_eq!( -1_i16 as i32, -1_i32);       // sign-extended
    assert_eq!( 65535_u16 as i32, 65535_i32); // zero-extended
    assert_eq!( 1000_i16 as u8, 232_u8);

    assert_eq!(2_u16.pow(4), 16);               // expatiation
    assert_eq!((-4_i32).abs(), 4);              // absolute value
    assert_eq!((0b101101_u8).count_one(), 4);   // population count
}
