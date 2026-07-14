use crate::types::*;
use std::vec;

pub fn run() {
    println!("### Text examples");

    let t1 = t_text!("hello");
    let t2 = t_text!("world".to_string());
    let t3 = t_text!(Text::from("!!!"));

    let values: Vec<Box<dyn Val>> = vec![box_val!(t1), box_val!(t2), box_val!(t3)];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
