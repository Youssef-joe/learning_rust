fn main() {
    // Variables and data types
    let mut x: i32 = 5;
    x += 1;
    println!("x = {}", x);

    let y: f64 = 3.14;
    println!("y = {}", y);

    let name: &str = "Rust";
    println!("Hello, {}!", name);

    // Functions
    let result = add(10, 20);
    println!("10 + 20 = {}", result);

    // Control flow
    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is not greater than 5");
    }

    for i in 1..=3 {
        println!("Loop: {}", i);
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}