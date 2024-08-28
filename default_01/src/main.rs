struct _StructA {}

#[derive(Default, PartialEq, Debug)]
struct StructB {}

#[derive(Default)]
struct StructC {}

fn main() {
    // implements Default automatically:
    let a: (u32, u32) = Default::default();
    assert_eq!(a, (0, 0));

    // let b: _StructA = Default::default(); // error

    let b = StructB::default();
    assert_eq!(b, StructB {});

    let _c: StructC = Default::default();
}
