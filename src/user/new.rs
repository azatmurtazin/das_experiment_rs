use super::User;

impl User {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}
