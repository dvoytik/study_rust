// A tuple enum varian with parentheses can create fn() -> EnumType
#[derive(Debug)]
enum E1 {
    V1(),
}

// That's actually a feature of tuple structs!
struct TS();

fn main() {
    let ts1 = TS; // type: extern "rust-call" TS() -> TS
    let _ts2 = ts1(); // type: TS
    let _ts3 = TS(); // type TS

    let a = E1::V1(); // type: E1
    let b = E1::V1; // type: 'extern "rust-call" V1() -> E1' which 'impl Fn() -> E1'
    println!("r = {:?}", b());
    println!("r = {:?}", a);
}
// This is documented here: https://github.com/rust-lang/rfcs/blob/master/text/1506-adt-kinds.md#tuple-structs
