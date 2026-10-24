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

#[derive(Debug)]
enum Thing {
    Letter,
    Packet(i64),
    Crate(i64, i64),
}
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn postage (t: &Thing) -> i64 {
    match t {
        Thing::Letter => 1,
        Thing::Packet(_) => 2,
        Thing::Crate(x, y) => x + y,
    }
}

// enum with error handling
enum Option<T> {Some(T), None}
enum Result<T, E> {Ok(T), Err(E)}
// Use Option when absence needs no explanation: v.get(99) is None.
// Use Result when the caller deserves a reason. Both are ordinary values that you return,
// store and match.
// Because match must be exhaustive,
// you cannot reach the success value without also writing the failure arm

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

    let a = Thing::Letter;
    let b = Thing::Packet(20);
    let c = Thing::Crate(1989, 2001);
    println!("debugging a -> {:?}", a);
    println!("debugging b -> {:?}", b);
    println!("debugging c -> {:?}", c);

    if let Thing::Crate(kg, _) = c {
        println!("heave crate: {}", kg);
    }

    fn parse_port(text: &str) -> std::result::Result<u16, String> {
        let clean = text.trim();
        let port = match clean.parse::<u16>() {
            Ok(p) => p,
            Err(_) => return Err(format!("bad port: {}", clean)),
        };
        if port == 0 {
            return Err(String::from("port 0 is reserved"));
        }
        Ok(port)
    }


}