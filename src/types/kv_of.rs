use crate::types::Text;

use super::Val;
use std::fmt;

pub struct KvOf<T: Val> {
    k: Text,
    v: T,
}

impl<T: Val> KvOf<T> {
    pub fn new(k: Text, v: T) -> Self {
        Self { k, v }
    }
}

impl<T: Val> Val for KvOf<T> {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl<T: Val> fmt::Display for KvOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}:{})", self.k, self.v)
    }
}

impl<T: Val> fmt::Debug for KvOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fty = std::any::type_name::<T>();
        let ty = fty.split("::").last().unwrap_or(fty);

        write!(f, "KvOf<{}>({:?},{:?})", ty, self.k, self.v)
    }
}
