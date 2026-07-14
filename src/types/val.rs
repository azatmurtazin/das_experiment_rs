use dyn_clone::{DynClone, clone_trait_object};
use std::fmt::{Debug, Display};

pub trait Val: Display + Debug + DynClone {
    fn display(&self);
    fn inspect(&self);
}

clone_trait_object!(Val);

#[macro_export]
macro_rules! box_val {
    ($val:expr) => {
        Box::new($val) as Box<dyn Val>
    };
}
