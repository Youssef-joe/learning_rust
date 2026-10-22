use std::collections::HashMap;

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
    // make_loud(&mut before);
    println!("before -> , {}", before);
    // println!("after -> , {}", make_loud(&before));
    println!("count ->, {}", vowel_count(&before));


    // let's try out some vectors here
    let mut nums: Vec<i64> = Vec::new();
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in &names {
        println!("{}", name);
    }


    // doing some hashmaps
    let mut stock: HashMap<&str, i32> = HashMap::new();
    stock.insert("Bob", 1);
    stock.insert("Frank", 2);
    match stock.get("Bob") {
        Some(value) => { println!("Founded value: {}", value) },
        None => println!("There is no value")
    }
    println!("stock: {:?}", stock);
    println!("{}", stock.len());

    let votes = vec![1, 2, 3];
    let names = vec!["Bob", "Frank", "Ferris"];
    let mut whom: HashMap<i64, &str> = HashMap::new();
    for v in &votes {
        println!("{}", *v);
        *whom.entry(*v).or_insert("0") = "!";
    }

    let scores = vec![1, 2, 3];
    let num = scores.iter().count();
    let cnt: i64 = scores.iter().sum();

    println!("here's the count -> {}", cnt);
    println!("here's the number -> {}", num );

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
        let mut num = 0;
    for n in s.chars() {
        if n == 'a' {
            num+=1;
        }
    }
        return num
}

// Change the caller's String in place: make it uppercase, then make sure
// it ends with '!' (add one only if it does not already end in '!').
fn make_loud(s: &mut String) {
    *s = s.to_uppercase();
    if !s.ends_with('!') {
        s.push('!');
    }
}

