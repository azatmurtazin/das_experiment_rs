use crate::types::{AnyKv, AnyList, Int, KvOf, ListOf, Text, Val};
use std::vec;

pub fn run() {
    println!("### Dynamic types example");

    let i1 = Int::new(42);
    let i2 = Int::new(123);

    let t1 = Text::new("hello".to_string());
    let t2 = Text::new("world".to_string());

    let l1 = AnyList::new(vec![Box::new(t1.clone()), Box::new(i1.clone())]);
    let l2 = ListOf::new(vec![i1.clone(), i2.clone()]);
    let l3 = ListOf::new(vec![t1.clone(), t2.clone()]);

    let kv1 = KvOf::new(Text::new("five".to_string()), Int::new(5));
    let kv2 = KvOf::new(Text::new("seven".to_string()), Int::new(7));

    let kv3 = AnyKv::new(Text::new("qwe".to_string()), Box::new(i1.clone()));
    let kv4 = AnyKv::new(Text::new("rty".to_string()), Box::new(t1.clone()));

    let values: Vec<Box<dyn Val>> = vec![
        Box::new(i1),
        Box::new(t1),
        Box::new(t2),
        Box::new(l1),
        Box::new(l2),
        Box::new(l3),
        Box::new(kv1),
        Box::new(kv2),
        Box::new(kv3),
        Box::new(kv4),
    ];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
