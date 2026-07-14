use crate::types::*;
use std::vec;

pub fn run() {
    println!("### KeyValue examples");

    let kv1 = t_kv_of!(t_text!("one"), t_int!(1));
    let kv2 = t_kv_of!("two".to_string(), t_num!(2));
    let kv3 = t_kv_of!("three", t_text!("III"));

    let kv4 = t_kv_any!(t_text!("q"), box_val!(t_int!(42)));
    let kv5 = t_kv_any!("w".to_string(), box_val!(t_num!(5.55)));
    let kv6 = t_kv_any!("e", t_text!("hello"));

    let values: Vec<Box<dyn Val>> = vec![
        box_val!(kv1),
        box_val!(kv2),
        box_val!(kv3),
        box_val!(kv4),
        box_val!(kv5),
        box_val!(kv6),
    ];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
