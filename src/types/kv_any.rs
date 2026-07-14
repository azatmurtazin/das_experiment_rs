use crate::box_val;

use super::{Text, Val};
use std::fmt;

#[derive(Clone)]
pub struct KvAny {
    pub key: Text,
    pub val: Box<dyn Val>,
}

impl KvAny {
    pub fn new(key: Text, val: Box<dyn Val>) -> Self {
        Self { key, val }
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
        write!(f, "({}=>{})", self.key, self.val)
    }
}

impl fmt::Debug for KvAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KvAny({:?}=>{:?})", self.key, self.val)
    }
}

#[macro_export]
macro_rules! t_kv_any {
    ($key:expr, $val:expr) => {
        KvAny::from((Text::from($key), $val))
    };
}
