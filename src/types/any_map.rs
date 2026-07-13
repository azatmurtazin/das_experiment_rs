use super::{AnyKv, Text, Val};
use std::{collections::HashMap, fmt};

#[derive(Clone)]
pub struct AnyMap(HashMap<Text, Box<dyn Val>>);

impl AnyMap {
    pub fn new(v: Vec<AnyKv>) -> Self {
        let mut m: HashMap<Text, Box<dyn Val>> = HashMap::new();
        v.iter().for_each(|kv| {
            m.insert(kv.key(), kv.val());
        });
        Self(m)
    }
}

impl Val for AnyMap {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl fmt::Display for AnyMap {
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

impl fmt::Debug for AnyMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|(k, v)| format!("{:?}=>{:?}", k, v))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "AnyMap{{{}}}", s)
    }
}
