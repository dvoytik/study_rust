//////////////////////////////////////////////////////////////////////////////////////////////////

/// simple callback
fn simple_callback() {
    println!("simple_callback() called");
}

fn run_a_callback(cb: fn()) {
    cb();
}

// we can return a function - fn() type
fn return_simple_callback() -> fn() {
    simple_callback
}

// return a function defined internally
fn return_simple_callback2() -> fn() {
    fn simple_callback2() {
        println!("simple_callback2() called");
    }
    simple_callback2
}

fn pass_one_to_callback(cb: fn(i: i32)) {
    cb(1);
}

// Callback as generic function object
fn run_closure<CB: Fn()>(cl: CB) {
    cl();
}

// Same as previous, but using impl
fn run_closure2(cl: impl Fn()) {
    cl();
}

// Closure accepts two parameters
fn run_closure3(a: u8, b: u8, cl: impl Fn(u8, u8)) {
    cl(a, b);
}

// cl object contains mutable captured variable, that's why it must be mut
fn run_mut_closure(mut cl: impl FnMut()) {
    cl();
}

//////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
    let my_simple_callback = simple_callback;
    run_a_callback(my_simple_callback);
    run_a_callback(simple_callback);
    // return fn() and execute it:
    return_simple_callback()();
    return_simple_callback2()();
    // FYI, this does nothing:
    return_simple_callback();
    // closure is coerced to fn type if it doesn't capture any variables:
    run_a_callback(|| println!("closure is coerced to fn!"));

    // This won't work because closure cannot be coerced to fn() due to capturing variable s
    //let s: String = "test string".into();
    //run_a_callback(|| println!("anonymous callback also works {}", s));

    // this won't work because it expects fn()
    // run_a_callback(|i| println!("closure is coerced to fn(i), i={}!", i));

    // this works because it expects fn(i: i32)
    pass_one_to_callback(|i| println!("closure is coerced to fn(i), i={}!", i));

    run_closure(|| println!("a very simple closure"));
    run_closure2(|| println!("a very simple closure"));
    run_closure3(0xff, 0xaa, |a, b| {
        println!("closure with two params: {:x}, {:x}", a, b)
    });

    let s: String = "test string".into();
    run_closure(|| println!("anonymous callback with captured String: {}", s));

    // This won't work for Fn() closure because the captured variable is mutable:
    // let mut s_mut: String = "test string".into();
    // run_closure(|| {
    //     s_mut.push('!');
    //     println!("anonymous callback with captured String: {}", s_mut)
    // });

    // For mutable captured variables, closure must be FnMut()
    let mut s_mut: String = "test string".into();
    run_mut_closure(|| {
        s_mut.push('!');
        println!("anonymous callback with captured mut String: {}", s_mut)
    });
}
