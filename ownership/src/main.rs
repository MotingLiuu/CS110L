fn main() {
    // s is not valid here, because it's not yet declared
    let s = "hello"; // s is valid from this point forward

    let string_s = String::from("hello");
    
    let mut mutable_s = String::from("hello");
    mutable_s.push_str(", world"); 
    println!("{}", s);
    
    // let s1 = String::from("hello");
    // let s2 = s1; // move s1 into s2 rather than shallow copy
    
    let s1 = String::from("hello");
    let s2 = s1.clone(); // use clone to make a deep copy of s1, clone is expensive

    println!("s1 = {}, s2 = {}", s1, s2); 
    
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // the copy of stack-only data is quick to make. x and y are both valid.
    
    
    let s = String::from("hello");
    
    takes_ownership(s); // s is moved into the function, and no longer valid here
                        // 
    let x = 5;
    
    makes_copy(x); // i32 implements the Copy trait, x does not move into the funciton,
                   // it's ok to use x afterward.
    
}   // this scope is over, and s is no longer valid

fn takes_ownership(some_string: String) { // some_string comes into scope,
    println!("{}", some_string);
} // some_string goes out of scope and 'drop' is called.
  // The backing memo is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Some_integer goes out of scope, but i32 is Copy, so nothing happens.

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// The ownership of a var follows the same pattern every time:
// Assigning a value to another varibale moves it. when a var that includes  data on the heap goes out of scope
// the value will be cleaned up by drop unless ownership of the data has been moved to another var.