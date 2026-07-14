use crate::box_val;

use super::Val;
use std::fmt;

#[derive(Clone)]
pub struct ListAny(Vec<Box<dyn Val>>);

impl ListAny {
    pub fn new(v: Vec<Box<dyn Val>>) -> Self {
        Self(v)
    }
}

impl From<Vec<Box<dyn Val>>> for ListAny {
    fn from(v: Vec<Box<dyn Val>>) -> Self {
        Self::new(v)
    }
}

impl<T: Val + Clone + 'static> From<Vec<T>> for ListAny {
    fn from(v: Vec<T>) -> Self {
        let bv = v.iter().map(|x| box_val!(x.clone())).collect();
        Self::new(bv)
    }
}

impl Val for ListAny {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl fmt::Display for ListAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "[{}]", s)
    }
}

impl fmt::Debug for ListAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "ListAny[{}]", s)
    }
}

impl IntoIterator for ListAny {
    type Item = Box<dyn Val>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[macro_export]
macro_rules! t_list_any {
    ( $( $item:expr ),* $(,)? ) => {
        ListAny::new(vec![ $( box_val!($item) ),* ])
    };
}
