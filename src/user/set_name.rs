use super::User;

impl User {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
