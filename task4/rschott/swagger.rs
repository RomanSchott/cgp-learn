use std::fmt;


struct Swagger<T: fmt::Display> {
    d: T,
}


impl<T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#swag {} #yolo ", self.d)
    }
}

fn main() {
    let s = Swagger { d: 5346.6367 };
    println!("{}", s);
    let r = Swagger { d: "hallo!" };
    println!("{}", r);
}
