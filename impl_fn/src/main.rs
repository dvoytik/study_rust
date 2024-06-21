// An enum varian with parentheses can create fn() -> EnumType
#[derive(Debug)]
enum E1 {
    V1(),
}

fn main() {
    let a = E1::V1(); // type: E1
    let b = E1::V1; // type: 'extern "rust-call" V1() -> E1' which 'impl Fn() -> E1'
    println!("r = {:?}", b());
    println!("r = {:?}", a);
}
