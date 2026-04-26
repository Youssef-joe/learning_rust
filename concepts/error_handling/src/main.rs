use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Using Result with match
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Using unwrap
    // let f2 = File::open("hello2.txt").unwrap();

    // Using expect
    // let f3 = File::open("hello3.txt").expect("Failed to open hello3.txt");

    println!("File opened successfully");
}