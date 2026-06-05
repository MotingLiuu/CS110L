#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(3.14),
    ]; // all the enum variants will be considered the same type.
    
    println!("row:{:?}", row);
}