struct MyStruct {
    f: u32,
}

impl AsRef<u32> for MyStruct {
    fn as_ref(&self) -> &u32 {
        &self.f
    }
}

fn main() {
    let s = String::new();
    // let sl = s.as_ref(); // error: multiple imple of AsRef require type annotation
    let sl1: &[u8] = s.as_ref();
    assert_eq!(s.as_bytes(), sl1);

    let sl2: &str = s.as_ref();

    let sl3: Box<str> = <String as AsRef<str>>::as_ref(&s).into();
    let sl4: Box<str> = sl2.into();
    assert_eq!(sl3, sl4);

    let ms = MyStruct { f: 1 };
    assert_eq!(ms.as_ref(), &1);
    assert_eq!(*ms.as_ref(), 1);

    println!("Hello, world!");
}
