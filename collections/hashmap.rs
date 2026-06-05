use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let field_name = String::from("Color");
    let filed_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, filed_value); // This would transfer the ownership of field_name and filed_value to map

    println!("The map is: {:?}", map);   
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // get would return Option<V>, the type of score is Some(10)
    
    println!("The score of Blue is: {}", score.unwrap()); 
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // entry would return a mutable reference to a value in the map
                                                        // if the key is not in the map, it would insert the key and value
                                                        // and return a mutable reference to the value
    scores.entry(String::from("Blue")).or_insert(50);
    
    println!("The map is: {:?}", scores);
    

    let text = "hello world wanderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("The map is: {:?}", map);
}