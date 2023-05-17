#[derive(Debug)]
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {

    let user1= build_user(
        "Tom",
        "someone@example.com",
    );
    
    let user2 = User {
        email: "another@example.com",
        // username: String::from("Anna"),
        ..user1
    };

    println!("{:?}", user1);
    println!("{:?}", user2);

}

fn build_user(username: &str, email: &str) -> User {
    User {
        active: true,
        username,
        sign_in_count: 1,
        email,
    }
}
