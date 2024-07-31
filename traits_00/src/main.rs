trait SimpleTrait {
    fn method(&self);
}

impl SimpleTrait for i32 {
    fn method(&self) {
        println!("method() for i32 called. Value: {}", *self);
    }
}

impl SimpleTrait for f32 {
    fn method(&self) {
        println!("method() for f32 called. Value: {}", *self);
    }
}

fn main() {
    let mut a: &dyn SimpleTrait;
    a = &2;
    a.method();
    a = &32.33;
    a.method();
}
