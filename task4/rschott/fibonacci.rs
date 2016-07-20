use std::iter::Iterator;

struct Fibonacci {
    f: u32,
    i: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new = self.f + self.i;
        self.f = self.i;
        self.i = new;
        Some(self.f)
    }
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { f: 1, i: 1 }
    }
}
fn main() {
    for i in Fibonacci::new().take(20) {
        println!("{}", i);
    }
}
