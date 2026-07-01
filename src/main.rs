mod user;

use user::User;

fn main() {
    let mut user = User::new("John".to_string(), 30);
    println!("user = {:}", user);

    user.set_name("Mike".to_string());
    user.set_age(44);

    println!("user = {:}", user);
}
