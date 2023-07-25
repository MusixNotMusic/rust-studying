struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@email.com"),
        username: String::from("someUsername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1.email ==> {}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!("user1.email ==> {}", user1.email);

    let email = String::from("123456@qq.com");
    let username = String::from("i am your father");
    let user2 = build_user(email, username);

    println!("user2.email ==> {}", user2.email);

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user3.email ==> {}", user3.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
