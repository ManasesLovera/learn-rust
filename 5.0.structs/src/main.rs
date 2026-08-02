struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // Using field init shorthand
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("Is user1 active? '{0}'", user1.active);

    user1.email = String::from("anotheremail@example.com");

    println!("user1.email = {0}", user1.email);

    let mut user2 = build_user(
        String::from("myemail@example.com"),
        String::from("myusername"),
    );

    println!("user2 email: {0}", user2.email);
}
