use crate::types::*;
use std::vec;

pub fn run() {
    println!("### List example");

    let l1 = AnyList::new(vec![Box::new(t_text!("hello")), Box::new(t_int!(42))]);
    let l2 = ListOf::new(vec![t_int!(42), t_int!(123)]);
    let l3 = ListOf::new(vec![t_text!("hello"), t_text!("world")]);

    let values: Vec<Box<dyn Val>> = vec![Box::new(l1), Box::new(l2), Box::new(l3)];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
