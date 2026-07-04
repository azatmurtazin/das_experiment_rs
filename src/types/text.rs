use super::Val;
use std::fmt;

#[derive(Clone)]
pub struct Text {
    v: String,
}

impl Text {
    pub fn new(v: String) -> Self {
        Self { v }
    }
}

impl Val for Text {
    fn inspect(&self) {
        println!("{}", self)
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text({})", self.v)
    }
}
