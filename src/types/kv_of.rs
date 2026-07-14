use super::{Text, Val};
use std::fmt;

#[derive(Clone)]
pub struct KvOf<T: Val> {
    pub key: Text,
    pub val: T,
}

impl<T: Val + Clone> KvOf<T> {
    pub fn new(key: Text, val: T) -> Self {
        Self { key, val }
    }
}

impl<T: Val + Clone> From<(Text, T)> for KvOf<T> {
    fn from(kv: (Text, T)) -> Self {
        let (k, v) = kv;
        Self::new(k, v)
    }
}

impl<T: Val + Clone> Val for KvOf<T> {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl<T: Val> fmt::Display for KvOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}=>{})", self.key, self.val)
    }
}

impl<T: Val> fmt::Debug for KvOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fty = std::any::type_name::<T>();
        let ty = fty.split("::").last().unwrap_or(fty);

        write!(f, "KvOf<{}>({:?}=>{:?})", ty, self.key, self.val)
    }
}

#[macro_export]
macro_rules! t_kv_of {
    ($key:expr, $val:expr) => {
        KvOf::from((Text::from($key), $val))
    };
}
