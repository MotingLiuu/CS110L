use std::fs::File;
use std::io::Read;
use std::io;

fn main() {
    // lel f = File::open("hello.txt").unwrap();
    // 
    // let f = File::open("hello.txt").expect("Failed to open hello.txt"); 
    // use expect to print personalized error message.
    // 
    // let s = read_username_from_file().unwrap();
    let s = match read_username_from_file() {
        Ok(s) => s,
        Err(e) => panic!("Problem reading the file: {}", e),
    };
    println!("{}", s);
}

fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
// 
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     }
//     
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
//     
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)  // use ? to return Result<String, io::Error>
           // If the value of the Result is Ok, the value is returned
           // If the value of the Result is Err, the function returns early and the error is returned
}