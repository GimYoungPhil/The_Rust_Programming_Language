fn main() {
    // 구조체 정의
    struct User {
        email: String,
        username: String,
        active: bool,
        sign_in_count: u32,
    }

    // 구조체 인스턴스 생성
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser"),
        ..user1,
    };

    println!("email: {}, name: {}, active: {}, count: {}", user1.email, user1.username, user1.active, user1.sign_in_count);
    println!("email: {}, name: {}, active: {}, count: {}", user2.email, user2.username, user2.active, user2.sign_in_count);

}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
