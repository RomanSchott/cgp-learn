use std::ops::{Add, Mul};

fn main() {
    println!("Sum: {}, Mul: {}", calculate(2.5, 3.0).0,calculate(2.5, 3.0).1,);
}
fn calculate<T: Add + Mul + Copy>(a: T, b: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (a + b, a * b)
}
