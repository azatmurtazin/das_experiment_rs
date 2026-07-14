use dyn_clone::{DynClone, clone_trait_object};
use std::fmt::{Debug, Display};

pub trait Val: Display + Debug + DynClone {
    fn display(&self);
    fn inspect(&self);
}

clone_trait_object!(Val);
