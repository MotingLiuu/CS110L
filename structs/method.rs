#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {                    // impl block will be associated with the struct
                                    // 
   fn area(&self) -> u32 {
        self.width * self.height
    }
   
   fn square(size: u32) -> Self {   // Self is the type of the struct
       // Associated functions that are not methods are often used for 
       // constructors. 
       // Use Rectangle::square(30) to create a new instance of Rectangle.
       Self {
           width: size,
           height: size,
       }
   } }

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle::square(30);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    )
}