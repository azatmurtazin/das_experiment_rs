use super::Val;
use std::fmt;

#[derive(Clone)]
pub struct ListOf<T: Val>(Vec<T>);

impl<T: Val> ListOf<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self(v)
    }
}

impl<T: Val + Clone> Val for ListOf<T> {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl<T: Val> fmt::Display for ListOf<T> {
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

impl<T: Val> fmt::Debug for ListOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fty = std::any::type_name::<T>();
        let ty = fty.split("::").last().unwrap_or(fty);

        let s = self
            .0
            .iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "ListOf<{}>({})", ty, s)
    }
}
