fn main() {
    assert_eq!(u64::MAX as u32, u32::MAX);
    assert_eq!(u32::MAX as u64, 4294967295_u64);

    assert_eq!(-1_i32 as u32, u32::MAX);

    assert_eq!(u64::MAX as i32, -1);

    assert_eq!(0xaaaa_aaaa_aaaa_aaaa_u64 as u8, 0xaa_u8);

    assert_eq!(-1_i8 as i64, u64::MAX as i64);

    assert_eq!(i32::MAX as u32, u32::MAX / 2);

    assert_eq!(true as u8, 1);

    assert_eq!(false as u128, 0_u128);
}
