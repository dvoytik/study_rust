use std::str::Chars;

#[derive(Default)]
struct Strings<'a> {
    strings: Vec<&'a str>,
    current_string: usize,
    current_iterator: Option<Chars<'a>>,
}

impl Iterator for Strings<'_> {
    // We must set the associated type
    type Item = char;

    fn next(&mut self) -> Option<char> {
        loop {
            if self.current_string >= self.strings.len() {
                return None;
            }
            if self.current_iterator.is_none() {
                self.current_iterator = Some(self.strings[self.current_string].chars());
            }
            let c = self.current_iterator.as_mut().unwrap().next();
            if c.is_none() {
                self.current_string += 1;
                self.current_iterator.take();
            } else {
                return c;
            }
        }
    }
}

fn main() {
    let mut strings = Strings::default();
    strings.strings.push("test");
    strings.strings.push("second");
    strings.strings.push("test");
    for c in strings {
        print!("{} ", c)
    }
}
