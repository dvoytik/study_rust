use std::mem;

// I had to make T: ?Sized to allow unsized types.
// Note that unsized typed can be accessed only via a reference &T
fn type_of<T: ?Sized>(_: &T) -> &str {
    std::any::type_name::<T>()
}

/// https://doc.rust-lang.org/std/primitive.slice.html
fn main() {
    let _array1 = [3];
    assert_eq!(type_of(&_array1), "[i32; 1]");
    assert_eq!(mem::size_of_val(&_array1), 4);

    let _array1 = [3_u8];
    assert_eq!(type_of(&_array1), "[u8; 1]");
    assert_eq!(mem::size_of_val(&_array1), 1);

    let array = [3, 5, 7, 11]; // type is [i32; 4]
    println!("array: {array:?}, its size is {}", mem::size_of_val(&array));
    assert_eq!(mem::size_of_val(&array), 4 * 4);
    // array: [3, 5, 7, 11]
    println!("sizeof(array) = {}", std::mem::size_of::<[i32; 4]>());
    // sizeof(array) = 16

    let array_i32 = [0; 10]; // i32 is default type
    println!("array_i32: {:?}", array_i32);
    // array_i32: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    let array_u8: [u8; 10] = [0x55; 10];
    println!("array: {:?}", array_u8);
    // array: [85, 85, 85, 85, 85, 85, 85, 85, 85, 85]

    // Arrays of sizes from 1 to 12 (inclusive) implement From<Tuple>, where Tuple is a homogenous tuple of appropriate length.
    // let array_from_tuple: [i32; 3] = (1, 2, 3).into();

    let slice_u8 = &array_u8[..]; // type &[u8]
    println!("slice: {:?}", slice_u8);

    // WARNING: This is not a slice but an array of one element of type RangeFrom<i32>
    let _slice = [0..];
    println!("typeof([0..]): {}", type_of(&_slice));
    let slice = &[0..]; // same as &_slice
    println!("slice: {:?}", slice);

    // TODO: RangeFrom
    // assert_eq!((2..), std::ops::RangeFrom { start: 2 });
    let arr = [0, 1, 2, 3, 4];
    assert_eq!(type_of(&arr), "[i32; 5]");
    assert_eq!(type_of(&arr[..]), "[i32]");
    assert_eq!(arr[..], [0, 1, 2, 3, 4]);
    assert_eq!(arr[..3], [0, 1, 2]);
    assert_eq!(type_of(&arr[..3]), "[i32]");
    assert_eq!(arr[..=3], [0, 1, 2, 3]);
    assert_eq!(arr[1..], [1, 2, 3, 4]);
    assert_eq!(type_of(&arr[1..]), "[i32]");
    assert_eq!(arr[1..3], [1, 2]);
    assert_eq!(arr[1..=3], [1, 2, 3]);
}
