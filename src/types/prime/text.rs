use super::super::Val;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Text(String);

impl Text {
    pub fn new(v: String) -> Self {
        Self(v)
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Text::new(s.to_string())
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Text::new(s)
    }
}

impl Val for Text {
    fn display(&self) {
        println!("{}", self)
    }

    fn inspect(&self) {
        println!("{:?}", self)
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text({:?})", self.0)
    }
}

#[macro_export]
macro_rules! t_text {
    ($val:expr) => {
        Text::from($val)
    };
}
