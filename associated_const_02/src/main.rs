use std::ops::{Add, Sub};

trait FibConst {
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
}

fn fib<T>(n: T) -> T
where
    T: FibConst + Sub<Output = T> + Add<Output = T> + Eq + Copy,
{
    match n {
        n if n == T::ZERO => T::ZERO,
        n if n == T::ONE => T::ONE,
        n => fib::<T>(n - T::ONE) + fib::<T>(n - T::TWO),
    }
}

impl FibConst for u32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
}

// f32 and f64 doesn't implement Eq
// impl FibConst for f64 {
//     const ZERO: Self = 0.0;
//     const ONE: Self = 1.0;
//     const TWO: Self = 2.0;
// }

impl FibConst for i16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
}

fn main() {
    println!("u32 fib(35): {}", fib::<u32>(35));
    // println!("u32 fib(35.0): {}", fib::<f64>(35.0));
    println!("u32 fib(15): {}", fib::<i16>(15));
}
