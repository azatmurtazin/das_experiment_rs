use std::fmt;

pub trait Val: fmt::Display + fmt::Debug {
    fn display(&self);
    fn inspect(&self);
}
