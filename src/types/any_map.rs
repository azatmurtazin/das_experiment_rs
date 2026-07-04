use super::Val;
use std::fmt;

pub struct AnyMap {
    v: Vec<Box<dyn Val>>,
}

impl AnyMap {
    pub fn new(v: Vec<Box<dyn Val>>) -> Self {
        Self { v }
    }
}

impl Val for AnyMap {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl fmt::Display for AnyMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .v
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "[{}]", s)
    }
}

impl fmt::Debug for AnyMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .v
            .iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "AnyMap([{}])", s)
    }
}
