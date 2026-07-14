use super::{Text, Val};
use std::fmt;

#[derive(Clone)]
pub struct KvOf<T: Val>(Text, T);

impl<T: Val + Clone> KvOf<T> {
    pub fn new(k: Text, v: T) -> Self {
        Self(k, v)
    }

    pub fn key(&self) -> Text {
        self.0.clone()
    }

    pub fn val(&self) -> T {
        self.1.clone()
    }
}

impl<T: Val + Clone> From<(Text, T)> for KvOf<T> {
    fn from(kv: (Text, T)) -> Self {
        let (k, v) = kv;
        Self::new(k, v)
    }
}

impl<T: Val + Clone> From<(String, T)> for KvOf<T> {
    fn from(kv: (String, T)) -> Self {
        let (k, v) = kv;
        Self::new(Text::from(k), v)
    }
}

impl<T: Val + Clone> From<(&str, T)> for KvOf<T> {
    fn from(kv: (&str, T)) -> Self {
        let (k, v) = kv;
        Self::new(Text::from(k), v)
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
        write!(f, "({}:{})", self.0, self.1)
    }
}

impl<T: Val> fmt::Debug for KvOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fty = std::any::type_name::<T>();
        let ty = fty.split("::").last().unwrap_or(fty);

        write!(f, "KvOf<{}>({:?},{:?})", ty, self.0, self.1)
    }
}

#[macro_export]
macro_rules! t_kv_of {
    ($key:expr, $val:expr) => {
        KvOf::from(($key, $val))
    };
}
