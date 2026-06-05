fn main() {
    let mut v: Vec<i32> = Vec::new();
   
    let v1 = vec![1, 2, 3]; // rust can infer the type and use macro to create vector
                            // 
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);                            
    
    println!("{:?}", v);
    println!("{:?}", v1);
    
    let v2 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v2[2];
    println!("The third element is: {}", third); // use & and [] to get a reference to an element
    
    match v2.get(2) { // use get to get and Option<&i32> to get a reference to an element
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element"),
    }
    
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(hunder) => println!("The hunder element is: {}", hunder),
        None => println!("There is no third element"),
    }
    
    for i in &v2 {
        println!("{}", i);
    } // use & to get a reference to an element and for loop to iterate over a vector
      // 
    
    for i in &mut v {
        *i += 50;
    } // use * to dereference a reference and for loop to iterate over a vector
    
    println!("The vector v is: {:?}", v);
    
    
}