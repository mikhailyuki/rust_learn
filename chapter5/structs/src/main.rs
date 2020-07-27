#[derive(Debug)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        username: String::from("Alice"),
        email: String::from("test@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("{}", user1.username);
    let user2 = User {
        username: String::from("Bob"),
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}", user2.username);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// fn build_user(mail: String, name: String) -> User {
//     User {
//         email: mail,
//         username: name,
//         sign_in_count: 1,
//         active: true,
//     }
// }
