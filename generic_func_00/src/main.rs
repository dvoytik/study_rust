use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;

// there is not much can be done with unrestricted type parameter
fn generic_func0<T>(p: T) -> T {
    p
}

// That's why usually there is a bound with another trait(s)
// T must be bound to Debug trait otherwise println!("{:?}") won't compile
fn print_param_value<T: Debug>(p: &T) {
    println!("input parameter is {:?}", p);
}

fn generic_add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn _top_ten0<T: Hash + Eq + Debug>(_e: &[T]) {
    todo!()
}

fn _top_ten1<T>(_e: &[T])
where
    T: Hash + Debug + Eq,
{
    todo!()
}

fn main() {
    // monomorphization of generic function generic_func0()
    let _ = generic_func0(1);
    let _ = generic_func0("string");
    let _ = generic_func0::<u8>(255);
    // let _ = generic_func0::<u32>(1_u8); // error: mismatched types

    let v = "str";
    print_param_value(&v);

    let _a = generic_add(1, 2);
    let _b = generic_add(1.0, 2.0);
    // let _c = generic_add(1.0, 2); // mismatched types
    // let _b = generic_add("str", "str"); // trait Add is not implemented for &str
    // let _b: String = generic_add(String::new(), String::new()); // trait Add is not implemented for String
}
