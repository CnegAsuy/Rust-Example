use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared_string = Rc::new(RefCell::new(String::from("Hello")));

    let reference = Rc::clone(&shared_string);
    
    {
        reference.borrow_mut().push_str(", world!");    
    }
    println!("Reference 2: {}", shared_string.borrow());
}
