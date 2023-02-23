fn main() {
    // 구조체 정의
    struct User {
        email: String,
        username: String,
        active: bool,
        sign_in_count: u32,
    }

    // 구조체 인스턴스 생성
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };

    // 인스턴스 필드값 변경
    user.email = String::from("someone123@example.com");

    println!("email: {}", user.email);
    println!("name: {}", user.username);
    println!("active: {}", user.active);
    println!("count: {}", user.sign_in_count);
}
