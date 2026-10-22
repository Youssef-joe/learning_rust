// Structs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// let's impl
struct Span {
    start: i32,
    end: i32,
}
impl Span {
    fn hour(start: i32) -> Span {
        Span{start, end: start + 1}
    }
}

struct Personin {
    start: i32,
    username: String,
    age: i32
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

    // using impl
    let launch = Span::hour(20);

    // Using enum
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}