fn main() {
    let mut s = String::new();
    
    let data = "hello world";
    let s = data.to_string(); // use to_string to convert a string literal to a String
                              // any type that inplements the Display trait can be converted to a String
    
    let s = "initial value".to_string();
    
    let mut s = String::from("initial value");
    s.push_str(", appended"); // push_str accepts a &str
                              // 

    let mut s = String::from("initial value");
    s.push('a'); // push accepts a char
                    // 

    let s1 = String::from("initial value");
    let s2 = String::from("iiiinitial value");
    let s3 = s1 + &s2; // + accepts &str, s1 can not be used after the + operator
                       //The + operator calls the add method on the String type, add(self, s: &str) -> String
                       //The type of &s2 is &string, but the compiler converts &string to &str
                       //
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // use format! to create a string
                                             // {} is a placeholder for a value
                                             // format! can take any number of values
                                             //
    println!("The result of the format! macro is: {}", s);
                                             
                                             
}