use super::Val;
use std::fmt;

#[derive(Clone)]
pub struct Num(f64);

impl Num {
    pub fn new(v: f64) -> Self {
        Self(v)
    }
}

impl Val for Num {
    fn display(&self) {
        println!("{:}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl From<f64> for Num {
    fn from(f: f64) -> Self {
        Self::new(f)
    }
}

impl From<i64> for Num {
    fn from(i: i64) -> Self {
        Self::new(i as f64)
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Num({:?})", self.0)
    }
}

#[macro_export]
macro_rules! t_num {
    ($val:expr) => {
        Num::from($val)
    };
}
