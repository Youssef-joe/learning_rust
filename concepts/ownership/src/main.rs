fn main() {
    // Ownership: values have a single owner
    let s1 = String::from("hello");
    let s2 = s1; // Ownership moves to s2, s1 is invalid
    // println!("{}", s1); // This would error
    println!("s2 = {}", s2);

    // Borrowing: references
    let s = String::from("world");
    let len = calculate_length(&s); // Borrow s
    println!("Length of '{}' is {}", s, len);

    // Mutable borrowing
    let mut s3 = String::from("mutable");
    change(&mut s3);
    println!("s3 = {}", s3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" borrow");
}