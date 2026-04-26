fn main() {
    // Immutable borrowing
    let s = String::from("hello");
    let len = calculate_length(&s); // Immutable reference
    println!("The length of '{}' is {}.", s, len);

    // Mutable borrowing
    let mut s2 = String::from("hello");
    change(&mut s2); // Mutable reference
    println!("Changed string: {}", s2);

    // Multiple immutable borrows
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);

    // Slices
    let s3 = String::from("hello world");
    let hello = &s3[0..5];
    let world = &s3[6..11];
    println!("hello: {}, world: {}", hello, world);

    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Slice: {:?}", slice);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}