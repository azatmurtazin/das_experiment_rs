use crate::types::*;
use std::vec;

pub fn run() {
    println!("### Map example");

    let m1 = MapOf::new(vec![
        t_kv_of!("two", t_int!(2)),
        t_kv_of!("five".to_string(), t_int!(5)),
    ]);
    let m2 = AnyMap::new(vec![
        AnyKv::new(t_text!("qwe"), Box::new(t_int!(42))),
        AnyKv::new(t_text!("rty"), Box::new(t_text!("hello"))),
    ]);

    let values: Vec<Box<dyn Val>> = vec![Box::new(m1), Box::new(m2)];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
