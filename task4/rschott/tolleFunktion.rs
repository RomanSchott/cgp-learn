fn main() {
    println!("Sum: {}, Mul: {}", calculate(2.5, 3.0).0,calculate(2.5, 3.0).1,);
}
fn calculate<T: std::ops::Add + std::ops::Mul + Copy>
    (a: T,
     b: T)
     -> (<T as std::ops::Add>::Output, <T as std::ops::Mul>::Output) {
    (a + b, a * b)
}
