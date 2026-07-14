use crate::types::*;
use std::vec;

pub fn run() {
    println!("### Map examples");

    let m1 = MapOf::new(vec![
        t_kv_of!("two", t_int!(2)),
        t_kv_of!("five".to_string(), t_int!(5)),
    ]);
    let m2 = AnyMap::new(vec![
        KvAny::new(t_text!("qwe"), box_val!(t_int!(42))),
        KvAny::new(t_text!("rty"), box_val!(t_text!("hello"))),
    ]);

    let values: Vec<Box<dyn Val>> = vec![box_val!(m1), box_val!(m2)];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
