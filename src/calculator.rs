use std::io::{stdin, stdout, Write};

// A basic calculator.
fn main() {
    let number_1: i32 = take_i32("First num: ");
    let number_2: i32 = take_i32("Second num: ");
    println!("\x1B[2J\x1B[1;1H");
    println!("\t1. Addition\n\t2. Substraction\n\t3. Multiplication\n\t4. Division\n");
    let operation_type = take_i32("Type of process: ");
    match operation_type {
        1 => println!("{number_1} + {number_2} = {result}", result = number_1 + number_2),
        2 => println!("{number_1} - {number_2} = {result}", result = number_1 - number_2),
        3 => println!("{number_1} * {number_2} = {result}", result = number_1 * number_2),
        4 => {
            if number_2 != 0 {
                println!("{number_1} / {number_2} = {result}", result = number_1 as f32 / number_2 as f32);
            } else {
                println!("Cannot divide {number_1} to 0");
            }
        },
        _ => println!("Please input a number between 1 and 4."),
    }
}

fn take_i32(prompt: &str) -> i32 {
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new(); 
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    
    // Use trim() to remove whitespace and newlines
    input.trim().parse().expect("Please input a number!")
}
