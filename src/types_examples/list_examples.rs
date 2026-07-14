use crate::types::*;
use std::vec;

pub fn run() {
    println!("### List examples");

    let l1 = AnyList::new(vec![box_val!(t_text!("hello")), box_val!(t_int!(42))]);
    let l2 = ListOf::new(vec![t_int!(42), t_int!(123)]);
    let l3 = ListOf::new(vec![t_text!("hello"), t_text!("world")]);

    let values: Vec<Box<dyn Val>> = vec![box_val!(l1), box_val!(l2), box_val!(l3)];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
