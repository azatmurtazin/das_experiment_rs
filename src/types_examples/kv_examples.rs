use crate::types::*;
use std::vec;

pub fn run() {
    println!("### KeyValue example");

    let kv1 = t_kv_of!(t_text!("two"), t_int!(2));
    let kv2 = t_kv_of!("five".to_string(), t_int!(5));
    let kv3 = t_kv_of!("seven", t_int!(7));

    let kv4 = AnyKv::new(t_text!("qwe"), Box::new(t_int!(42)));
    let kv5 = AnyKv::new(t_text!("rty"), Box::new(t_text!("hello")));

    let values: Vec<Box<dyn Val>> = vec![
        Box::new(kv1),
        Box::new(kv2),
        Box::new(kv3),
        Box::new(kv4),
        Box::new(kv5),
    ];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
