use crate::types::Text;

use super::Val;
use std::fmt;

#[derive(Clone)]
pub struct AnyKv {
    k: Text,
    v: Box<dyn Val>,
}

impl AnyKv {
    pub fn new(k: Text, v: Box<dyn Val>) -> Self {
        Self { k, v }
    }
}

impl Val for AnyKv {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl fmt::Display for AnyKv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.k, self.v)
    }
}

impl fmt::Debug for AnyKv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AnyKv({:?},{:?})", self.k, self.v)
    }
}
