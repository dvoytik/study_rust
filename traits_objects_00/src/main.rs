use std::io::Write;

fn write_hello(p: &mut dyn Write) {
    p.write_all(b"hello there").unwrap();
}

fn main() {
    let mut v = vec![0, 1];
    v.write_all(b"test").unwrap();

    // r is trait object
    let r: &mut dyn Write;
    r = &mut v; // automatically converts normal reference to trait object that is fat pointer -
                // (address_of_data, address_of_vtable)
    r.write_all(b" more").unwrap();
    assert_eq!(std::any::type_name_of_val(&r), "&mut dyn std::io::Write");
    write_hello(r);

    let mut b: Box<dyn Write>;
    let a = Box::new(v.clone());
    b = a; // automatically converts Box<Vec<u8>> into Box<dyn Write>
    b.write_all(b" even more.").unwrap(); // write_all() called via virtual table (vtable)
    assert_eq!(
        std::any::type_name_of_val(&b),
        "alloc::boxed::Box<dyn std::io::Write>"
    );
    write_hello(&mut b); // convert Box<dyn Write> to &dyn Write
}
