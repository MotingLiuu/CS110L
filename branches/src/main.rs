fn main() {
    let mut count = 0;
    'counting_up: loop { // specify a loop lable with a single quote
        println!("count: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            } 
            if count == 2 {
                break 'counting_up; // break the loop with a lable counting_up
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count = {}", count);
    
    for number in (1..4).rev() { // use rev to reverse the range
        println!("number: {}", number);
    }
}
