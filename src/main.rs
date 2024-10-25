//#![allow(unused)]
//#[allow(unused_imports)]
//use std::any::type_name_of_val;
fn main() {
    // Tried to assert_eq() func.
    /* 
    let x:String = String::from("12");
    println!("{:?}", assert_eq!(x, "11"));
    */
    
    // Tried to mutable variable
    /* 
    let mut x = 1;
    x += 2;
    assert_eq!(x, 4);
    println!("Success");
    */
    // mutable variables types are cannot able to change.
    // x = "12"; // is failed because i try to change the type of x, int32 to str.
    // but if i try to define the variables again it don't gives any error.
    /*
    let y = 12;
    println!("y is {y} and {:?}", type_name_of_val(&y));
    let y = "212";
    println!("y is {y} and {:?}", type_name_of_val(y));
    */
    /*
    // related to funcs on the line 38 and the line 45.
    println!("12 + 2 is {}" , add_two(12));
    println!("12 + 14 is {}" , add_two_number_each_other(12, 14));
    */
    
    // Working with bin, oct and hex.
    // bin = 0b, hex = 0x and oct is defined with 0o.
    /*
    let bin = 0b01101i32;
    let _hex = 0xDi32;
    let oct = 0o15i32;
    println!("Number 13 in;");
    println!("\tBinary is 0b{bin:b}");
    println!("\tHexadecimal is 0x{_hex:x}");
    println!("\tOctal is 0o{oct:o}");
    */
    // Work with arrays is the easiest way to do anything everytime :D
    // "for" is work like "foreach", it just like python. But in rust you should define the types it's so boring, but it's important to opimise ig.
    /*
    let x = [2,12,14];
    println!("{x:?}");
    for i in x {
        println!("{i}");
    }
    */
    // matrix?
    /*
    let x = [
        [true,true,false],
        [true,false,true],
        [true,true, false]
        ];
        for i in x {
            for y in i {
                print!("{}", (if y {1} else {0}))
            }
            println!("")
        }
    */
}

// I created a func named a add_two it takes an integer paramater and add 2 of this integer. 
// So this is show by "f(x)= x + 2" in math.
// I used #[allow(dead_code)] for ignore the warning for unused function 👍.
/*
#[allow(dead_code)]
fn add_two(x: i32) -> i32 {
    x + 2 
}
*/
// This func add the numbers which i take from paramaters and return them.
/*
#[allow(dead_code)]
fn add_two_number_each_other(x: i32, y: i32) -> i32 {
    x + y
}
*/