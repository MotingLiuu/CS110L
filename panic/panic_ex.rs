use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let f: u32 = File::open("hello.txt"); // expecting a u32, got Result<File, std::io::Error>
    // 
    // let f: u32 = File::open("hello.txt").unwrap(); // expecting a u32, got Result<File, std::io::Error>

    // let f: u32 = File::open("hello.txt").unwrap(); // expecting a u32, let f = match f { // When the result if Ok, this code will return inner file value out of Ok, 
    // let f: u32 = File::open("hello.txt").unwrap(); // expecting a u32,                   // then. assign the value to f.
    // let f: u32 = File::open("hello.txt").unwrap(); // expecting a u32,     OK(file) => file,
    // let f: u32 = File::open("hello.txt").unwrap(); // expecting a u32,     Err(error) => {
    // let f: u32 = File::open("hello.txt").unwrap(); // expecting a u32,         panic!("Problem opening the file: {}", error);
    // let f: u32 = File::open("hello.txt").unwrap(); // expecting a u32,     },
    // let f: u32 = File::open("hello.txt").unwrap(); // expecting a u32, };
    // 
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {}", e),
            },
            other_error => panic!("Problem opening the file: {}", error),
        },
    };
}

