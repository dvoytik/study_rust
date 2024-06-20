#[derive(Debug)]
enum E1 {
    V1(),
}

fn f1(_f: impl Fn() -> E1) {}

fn main() {
    let _c = f1(E1::V1);

    let b: Box<dyn Fn() -> E1> = Box::new(E1::V1);
    println!("r = {:?}", b());
}
