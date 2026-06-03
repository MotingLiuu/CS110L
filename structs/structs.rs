struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Declare a struct named Rectangle.
    // Declare a struct let height1 = 50;

    // Declare a struct println!(
    // Declare a struct     "The area of the rectangle is {} square pixels.",
    // Declare a struct     area(width1, height1)        
    // Declare a struct );
    // 

    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );
    // 

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
// 

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
// 

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}