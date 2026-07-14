use crate::types::*;

pub fn run() {
    println!("### List examples");

    let l1 = t_list_of![t_int!(42), t_int!(123)];
    let l2 = t_list_of![t_text!("hello"), t_text!("world")];

    let l3 = t_list_any![t_text!("hello"), t_int!(42)];
    let l4 = t_list_any![t_int!(0), t_num!(0.0), t_text!("0")];

    let values = t_list_any![l1, l2, l3, l4];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
