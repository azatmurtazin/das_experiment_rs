use super::{Text, Val};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct MapOf<T: Val>(HashMap<Text, T>);

impl<T: Val + Clone> MapOf<T> {
    pub fn new(v: HashMap<Text, T>) -> Self {
        Self(v)
    }
}

impl<T: Val + Clone> Val for MapOf<T> {
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
            .0
            .iter()
            .map(|(k, v)| format!("{}=>{}", k, v))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{{{}}}", s)
    }
}

impl<T: Val> fmt::Debug for MapOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fty = std::any::type_name::<T>();
        let ty = fty.split("::").last().unwrap_or(fty);

        let s = self
            .0
            .iter()
            .map(|(k, v)| format!("{:?}=>{:?}", k, v))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "MapOf<{}>{{{}}}", ty, s)
    }
}

#[macro_export]
macro_rules! t_map_of {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(
                temp_map.insert(Text::from($key), $value);
            )*
            MapOf::new(temp_map)
        }
    };
}
