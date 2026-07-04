mod types;
mod user;

use std::vec;

use types::{Int, Text, Val};
use user::User;

use crate::types::List;
use crate::types::ListOf;

fn main() {
    let mut user = User::new("John".to_string(), 30);
    println!("user = {:}", user);

    user.set_name("Mike".to_string());
    user.set_age(44);

    println!("user = {:}", user);

    let i1 = Int::new(42);
    let i2 = Int::new(123);
    let t1 = Text::new("hello".to_string());
    let t2 = Text::new("world".to_string());
    let l1 = List::new(vec![Box::new(t1.clone()), Box::new(t2.clone())]);
    let l2 = ListOf::new(vec![i1.clone(), i2.clone()]);

    let values: Vec<Box<dyn Val>> = vec![
        Box::new(i1),
        Box::new(t1),
        Box::new(t2),
        Box::new(l1),
        Box::new(l2),
    ];

    for value in values {
        value.inspect();
    }
}
