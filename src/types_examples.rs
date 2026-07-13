use crate::types::*;
use std::vec;

pub fn run() {
    println!("### Dynamic types example");

    let i1 = int!(42);
    let i2 = int!(123);

    let n1 = num!(12.34);
    let n2 = num!(0);

    let t1 = text!("hello");
    let t2 = text!("world".to_string());

    let l1 = AnyList::new(vec![Box::new(t1.clone()), Box::new(i1.clone())]);
    let l2 = ListOf::new(vec![i1.clone(), i2.clone()]);
    let l3 = ListOf::new(vec![t1.clone(), t2.clone()]);

    let kv1 = KvOf::new(text!("five"), int!(5));
    let kv2 = KvOf::new(text!("seven"), int!(7));

    let kv3 = AnyKv::new(text!("qwe"), Box::new(i1.clone()));
    let kv4 = AnyKv::new(text!("rty"), Box::new(t1.clone()));

    let m1 = MapOf::new(vec![kv1.clone(), kv2.clone()]);
    let m2 = AnyMap::new(vec![kv3.clone(), kv4.clone()]);

    let values: Vec<Box<dyn Val>> = vec![
        Box::new(i1),
        Box::new(n1),
        Box::new(n2),
        Box::new(t1),
        Box::new(t2),
        Box::new(l1),
        Box::new(l2),
        Box::new(l3),
        Box::new(kv1),
        Box::new(kv2),
        Box::new(kv3),
        Box::new(kv4),
        Box::new(m1),
        Box::new(m2),
    ];

    for value in values {
        value.display();
        value.inspect();
        println!();
    }
}
