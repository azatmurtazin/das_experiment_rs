use super::Val;
use std::fmt;

pub struct AnyList {
    v: Vec<Box<dyn Val>>,
}

impl AnyList {
    pub fn new(v: Vec<Box<dyn Val>>) -> Self {
        Self { v }
    }
}

impl Val for AnyList {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl fmt::Display for AnyList {
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

impl fmt::Debug for AnyList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .v
            .iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "AnyList([{}])", s)
    }
}
