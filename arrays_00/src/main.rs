use std::mem;

fn main() {
    let byte_array: &[u8; 2] = b"\x00\x01";
    assert_eq!(*byte_array, [0_u8, 1_u8]);
    assert_eq!(byte_array, &[0_u8, 1_u8]);

    let byte_array2: [u8; 2] = *b"\x00\x01";
    assert_eq!(byte_array2, [0_u8, 1_u8]);
    assert_eq!(byte_array, &byte_array2);
    assert_eq!(*byte_array, byte_array2);

    let byte_array3: [u8; 2] = [0, 1];
    assert_eq!(mem::size_of_val(&byte_array3), 2);
    assert_eq!(byte_array2, byte_array3);
    assert!(byte_array3 == byte_array2);
    assert!(byte_array3 <= byte_array2);
    assert!(byte_array3 >= byte_array2);
}
