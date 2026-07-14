use crate::types::*;

pub fn run() {
    println!("### Map examples");

    let m1 = MapOf::new(vec![
        t_kv_of!("two", t_int!(2)),
        t_kv_of!("five".to_string(), t_int!(5)),
    ]);
    let m2 = AnyMap::new(vec![
        t_kv_any!("qwe", box_val!(t_int!(42))),
        t_kv_any!("rty", box_val!(t_text!("hello"))),
    ]);

    let values = t_list_any![m1, m2];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
