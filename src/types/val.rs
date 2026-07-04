use std::fmt;

use dyn_clone::{DynClone, clone_trait_object};

pub trait Val: fmt::Display + fmt::Debug + DynClone {
    fn display(&self);
    fn inspect(&self);
}

clone_trait_object!(Val);
