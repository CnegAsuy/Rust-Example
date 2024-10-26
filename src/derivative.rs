// I put my rules on my own! It is my program
/*
#![deny(unused)]
fn main() {
    let x = 12;
    println!("{}", "sa");
}
*/

fn main() {
    println!("derivative of f(x) = x³ is {}", derivative(12));
}



fn f(x:f64) -> f64 {
    (x * x * x) as f64
}

fn derivative(y:i32) -> i32 {
    let dx = 0.0000000001;
    ((f(y as f64 + dx) - f(y as f64))/dx) as i32
}