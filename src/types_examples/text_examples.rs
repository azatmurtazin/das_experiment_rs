use crate::types::*;

pub fn run() {
    println!("### Text examples");

    let t1 = t_text!("hello");
    let t2 = t_text!("world".to_string());
    let t3 = t_text!(Text::from("!!!"));

    let values = t_list_any![t1, t2, t3];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
