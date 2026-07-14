use super::{Text, Val};
use std::{collections::HashMap, fmt};

#[derive(Clone)]
pub struct MapAny(HashMap<Text, Box<dyn Val>>);

impl MapAny {
    pub fn new(v: HashMap<Text, Box<dyn Val>>) -> Self {
        Self(v)
    }
}

impl Val for MapAny {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl fmt::Display for MapAny {
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

impl fmt::Debug for MapAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|(k, v)| format!("{:?}=>{:?}", k, v))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "MapAny{{{}}}", s)
    }
}

#[macro_export]
macro_rules! t_map_any {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(
                temp_map.insert(Text::from($key), box_val!($value));
            )*
            MapAny::new(temp_map)
        }
    };
}
