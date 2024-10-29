#![allow(unused)]
mod shapes;
use shapes::*;
fn main() {
    let x = Circle::new(Point::origin(), 13.2);
    println!("Circle\nArea: {}\nCircumference: {}\n", x.area(), x.circumference());
    let y = Rectangle::new(Point::origin(), Point::new(12.0, -25.0));
    println!("Rect\nArea: {}\nCircumference: {}", y.area(), y.circumference());
}