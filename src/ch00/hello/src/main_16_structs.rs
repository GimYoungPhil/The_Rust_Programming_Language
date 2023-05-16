#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let user1= build_user(
        String::from("Tom"),
        String::from("someone@example.com"),
    );
    
    let user2 = User {
        email: String::from("another@example.com"),
        // username: String::from("Anna"),
        ..user1
    };

    println!("{:?}", user1);
    println!("{:?}", user2);

}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        sign_in_count: 1,
        email,
    }
}
