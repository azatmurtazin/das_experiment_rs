use crate::types::*;

pub fn run() {
    println!("### Map examples");

    let m1 = t_map_of! {
        "two" => t_int!(2),
        "five" => t_int!(5),
    };

    let m2 = t_map_any! {
        "q" => t_int!(42),
        "w" => t_num!(11.22),
        "e" => t_text!("hello"),
    };

    let values = t_list_any![m1, m2];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
