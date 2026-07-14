use crate::box_val;

use super::{Text, Val};
use std::fmt;

#[derive(Clone)]
pub struct KvAny(Text, Box<dyn Val>);

impl KvAny {
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

impl From<(Text, Box<dyn Val>)> for KvAny {
    fn from(kv: (Text, Box<dyn Val>)) -> Self {
        let (k, v) = kv;
        Self::new(k, v)
    }
}

impl<T: Val + Clone + 'static> From<(Text, T)> for KvAny {
    fn from(kv: (Text, T)) -> Self {
        let (k, v) = kv;
        Self::new(k, box_val!(v))
    }
}

impl Val for KvAny {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl fmt::Display for KvAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}=>{})", self.0, self.1)
    }
}

impl fmt::Debug for KvAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KvAny({:?}=>{:?})", self.0, self.1)
    }
}

#[macro_export]
macro_rules! t_kv_any {
    ($key:expr, $val:expr) => {
        KvAny::from((Text::from($key), $val))
    };
}
