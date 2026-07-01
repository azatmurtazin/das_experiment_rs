use std::fmt;

pub struct User {
    name: String,
    age: u32,
}

impl User {
    pub fn new(name: String, age: u32) -> Self {
        Self {
            name: name,
            age: age,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User({}, {})", self.name, self.age)
    }
}
