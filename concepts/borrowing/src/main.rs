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


    let mut before = String::from("hello world!");
    make_loud(&mut before);
    println!("before -> , {}", before);
    // println!("after -> , {}", make_loud(&before));
    println!("count ->, {}", vowel_count(&before));
}
fn make_loud_correct(s: &mut String) -> String {
    *s = s.to_uppercase();

    if !s.ends_with('!') {
        s.push('!');
    }

    s.clone()
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}

// Count the vowels (a, e, i, o, u, in either case) in the borrowed text.
fn vowel_count(s: &str) -> usize {
    // TODO: walk the characters and count the vowels
    0
}

// Change the caller's String in place: make it uppercase, then make sure
// it ends with '!' (add one only if it does not already end in '!').
fn make_loud(s: &mut String) {
    // TODO: modify the String behind the mutable reference
}