use crate::types::*;
use std::vec;

pub fn run() {
    println!("### Numbers examples");

    let i1 = t_int!(42);
    let i2 = t_int!(123);

    let n1 = t_num!(12.34);
    let n2 = t_num!(0);

    let values: Vec<Box<dyn Val>> = vec![box_val!(i1), box_val!(i2), box_val!(n1), box_val!(n2)];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
