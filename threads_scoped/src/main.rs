use std::thread;

// docs: https://doc.rust-lang.org/std/thread/fn.scope.html

// This won't compile:
// fn main() {
//     let mut a = 1;
//
//     thread::spawn(|| {
//         a = 2;
//         println!("Hi from thread");
//         println!("a={}", &a);
//     })
//     .join()
//     .unwrap();
//
//     println!("Hi from main");
//     println!("a={}", &a);
// }

// thread::scope() forces spawned thread to join before thread::scope() returns,
// thus allows to mut borrow a.
fn main() {
    let mut a = 1;

    thread::scope(|s| {
        s.spawn(|| {
            a = 2;
            println!("Hi from thread");
            println!("a={}", &a);
        });
    });

    println!("Hi from main");
    println!("a={}", &a);
}
