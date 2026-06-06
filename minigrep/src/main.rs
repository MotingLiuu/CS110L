// This program need two terminal arguments.
// cargo run searchstring example-filename.txt
// 
use std::env;
use std::fs;

fn main() {
    // let args: Vec<String> = env::args().collect(); // use args() to get all arguments, and collect() to convert to Vec<String>
    // println!("{:?}", args); // just like c, the first argument is the program name itself
    // 
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {query} in {filename}\n");
    
    println!("In file {filename}\n");
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}
