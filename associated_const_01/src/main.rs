// you cannot have const inside enum
enum Test {
    _A1,
    _A2,
}
// you have to put it in the impl block
impl Test {
    const NUM: u32 = 100;
}

// use Self to define type for const
trait TestTrait {
    const NUM: u32 = 100;
    const CONST1: Self;
}

impl TestTrait for u32 {
    const NUM: u32 = 200;
    const CONST1: Self = 300;
}

// you cannot have const inside struct
struct TestStruct {}
// you have to put it in the impl block
impl TestStruct {
    const NUM: u32 = 500;
}

fn main() {
    println!("Enum const: {}", Test::NUM);
    println!("Trait const: {}", u32::CONST1);
    println!("Struct const: {}", TestStruct::NUM);
}
