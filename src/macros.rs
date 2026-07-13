macro_rules! int {
    ($val:expr) => {
        Int::from($val)
    };
}

macro_rules! num {
    ($val:expr) => {
        Num::from($val)
    };
}

macro_rules! text {
    ($val:expr) => {
        Text::from($val)
    };
}
