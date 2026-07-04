use crate::types::{AnyList, Int, ListOf, Text, Val};
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

    let values: Vec<Box<dyn Val>> = vec![
        Box::new(i1),
        Box::new(t1),
        Box::new(t2),
        Box::new(l1),
        Box::new(l2),
        Box::new(l3),
    ];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
