// This program need two terminal arguments.
// cargo run searchstring example-filename.txt
// 
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect(); // use args() to get all arguments, and collect() to convert to Vec<String>
    // println!("{:?}", args); // just like c, the first argument is the program name itself
    // 
    // let args: Vec<String> = env::args().collect();
    // let query = &args[1];
    // let filename = &args[2];

    // println!("Searching for {query} in {filename}\n");
    // 
    // println!("In file {filename}\n");
    // let contents = fs::read_to_string(filename)
    //     .expect("Something went wrong reading the file");
    // println!("With text:\n{}", contents);
    // 
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    // 
    let config = Config::new(env::args()).unwrap_or_else(
        |err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    );
    
    println!("Searching for {} in {}\n", config.query(), config.filename());
    // let _contents = fs::read_to_string(config.filename)
    //     .expect("Something went wrong reading the file");
    //     
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

