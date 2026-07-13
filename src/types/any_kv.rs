use super::{Text, Val};
use std::fmt;

#[derive(Clone)]
pub struct AnyKv(Text, Box<dyn Val>);

impl AnyKv {
    pub fn new(k: Text, v: Box<dyn Val>) -> Self {
        Self(k, v)
    }

    pub fn key(&self) -> Text {
        self.0.clone()
    }

    pub fn val(&self) -> Box<dyn Val> {
        self.1.clone()
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
        write!(f, "({},{})", self.0, self.1)
    }
}

impl fmt::Debug for AnyKv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AnyKv({:?},{:?})", self.0, self.1)
    }
}
