#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {   // implement fn x for all of the generic type
   fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {   // implement distance_from_origin just for Point<f32>
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
    
    println!("Integer Point is {integer:?}");
    println!("Float Point is {float:?}");
    println!("x of Integer Point is {}", integer.x());
    println!("x of Float Point is {}", float.x());
    
    let intoat = Point2 {x: 5, y: 4.0};
    println!("Intoat Point is {intoat:?}");

}