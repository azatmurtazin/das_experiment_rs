use crate::types::*;

pub fn run() {
    println!("### Numbers examples");

    let i1 = t_int!(42);
    let i2 = t_int!(123);

    let n1 = t_num!(12.34);
    let n2 = t_num!(0);

    let values = t_list_any![i1, i2, n1, n2];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
