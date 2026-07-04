use super::Val;
use std::fmt;

pub struct MapOf<T: Val> {
    v: Vec<T>,
}

impl<T: Val> MapOf<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self { v }
    }
}

impl<T: Val> Val for MapOf<T> {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl<T: Val> fmt::Display for MapOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .v
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "[{}]", s)
    }
}

impl<T: Val> fmt::Debug for MapOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fty = std::any::type_name::<T>();
        let ty = fty.split("::").last().unwrap_or(fty);

        let s = self
            .v
            .iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "MapOf<{}>({})", ty, s)
    }
}
