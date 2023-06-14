//////////////////////

/// simple callback
fn simple_callback() {
    println!("simple_callback() called");
}

fn run_simple_callback(cb: fn()) {
    cb();
}

// we can return fn type
fn return_simple_callback() -> fn() {
    simple_callback
}

// Callback as generic function object
fn run_closure<CB: Fn()>(cl: CB) {
    cl();
}

// Same as previous, but using impl
fn run_closure2(cl: impl Fn()) {
    cl();
}

fn main() {
    let my_simple_callback = simple_callback;
    run_simple_callback(my_simple_callback);
    run_simple_callback(simple_callback);
    // return fn() and execute it:
    return_simple_callback()();
    // closure is coerced to fn type if it doesn't capture any variables:
    run_simple_callback(|| println!("closure is coerced to fn!"));
    return_simple_callback();
    // This won't work because closure cannot be coerced due to capturing variable s
    //let s: String = "test string".into();
    //run_simple_callback(|| println!("anonymous callback also works {}", s));
    //
    run_closure(|| println!("very simple closure"));
    run_closure2(|| println!("very simple closure"));
    let s: String = "test string".into();
    run_closure(|| println!("anonymous callback with captured String: {}", s));
    // This won't work for Fn() closure because the captured variable is mutable:
    // let mut s_mut: String = "test string".into();
    // run_closure(|| {
    //     s_mut.push('!');
    //     println!("anonymous callback with captured String: {}", s_mut)
    // });
}
