use super::Val;
use std::fmt;

pub struct List {
    v: Vec<Box<dyn Val>>,
}

impl List {
    pub fn new(v: Vec<Box<dyn Val>>) -> Self {
        Self { v }
    }
}

impl Val for List {
    fn inspect(&self) {
        println!("{}", self)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .v
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "List({:})", s)
    }
}
