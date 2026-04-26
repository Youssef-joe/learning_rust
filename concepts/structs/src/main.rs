// Structs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Enums
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Using struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("User: {} <{}>", user1.username, user1.email);

    // Using enum
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}