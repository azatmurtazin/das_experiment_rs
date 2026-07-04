use std::fmt;

pub trait Val: fmt::Display {
    fn inspect(&self);
}
