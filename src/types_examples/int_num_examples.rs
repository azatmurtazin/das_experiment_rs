use crate::types::*;
use std::vec;

pub fn run() {
    println!("### Numbers example");

    let i1 = t_int!(42);
    let i2 = t_int!(123);

    let n1 = t_num!(12.34);
    let n2 = t_num!(0);

    let values: Vec<Box<dyn Val>> = vec![Box::new(i1), Box::new(i2), Box::new(n1), Box::new(n2)];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
