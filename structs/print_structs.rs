#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("rect1 is {:?}", rect1); // {:?} is the debug format specifier
    println!("rect1 is {:#?}", rect1); // {:#?} is the debug format specifier
                                       // Debug format would be ignored in when 
                                       // running dead code analysis.
    // println! takes a reference
}