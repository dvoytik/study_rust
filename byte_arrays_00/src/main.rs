use std::fmt::{self, Display};

struct ByteSlice<'a>(&'a [u8]);

impl<'a> Display for ByteSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in self.0 {
            write!(f, "\\x{:02x}", b)?
        }
        Ok(())
    }
}

fn main() {
    let byte_array = ByteSlice(b"\x00\x01");
    println!("byte slice: {}", byte_array);
}
