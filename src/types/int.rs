use super::Val;
use std::fmt;

#[derive(Clone)]
pub struct Int(i64);

impl Int {
    pub fn new(v: i64) -> Self {
        Self(v)
    }
}

impl From<i64> for Int {
    fn from(i: i64) -> Self {
        Self::new(i)
    }
}

impl Val for Int {
    fn display(&self) {
        println!("{:}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Int({:?})", self.0)
    }
}
