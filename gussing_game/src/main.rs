use std::cmp::Ordering;
use std::io; //Rust would bring some libraries into scope by default, which called prelude. 

use rand::Rng;

fn main() {
    println!("Guss the number!");

    // rand::thread_rng() return a random numver generator.
    let secret_number = rand::thread_rng().gen_range(1..=100); // gen_range takes two arguments, the first one is the lower bound, and the second one is the upper bound.

    loop{
        println!("Please input your guess.");


        let mut guess = String::new(); // use let to create a variable. mut means the variable is mutable.
         
        io::stdin() // stdin returns an instance of std::io::Stdin, which is a handle to the standard input of the terminal.
            .read_line(&mut guess) // read_line take whatever is in stdin and put it into guss. without overwriting the previous value.
             // read_line also return a Result, which is an enum that can either be Ok or Err.
             // Ok means the operation is successful, and Err means the operation is unsuccessful. And contains a value of the error.
             // Result has an except method. If Result is Err, it would cause the program to crash and display the message in the except.
            .expect("Failed to read line"); // & indicates that the argument is a reference. References are immutable by default.
         
         // The guess contain \n, trim method of string would remove  any whitespace at the beginning and end.
         // The parse method of string would convert the string to another type. : u32 tells the compiler that the string is a number.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // use mathc to handle the result of parse. The Ok value will match the 1st arm, and return num.

        println!("You guessed: {guess}"); 
         
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
} 
