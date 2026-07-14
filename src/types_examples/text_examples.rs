use crate::types::*;
use std::vec;

pub fn run() {
    println!("### Text example");

    let t1 = t_text!("hello");
    let t2 = t_text!("world".to_string());

    let values: Vec<Box<dyn Val>> = vec![Box::new(t1), Box::new(t2)];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
