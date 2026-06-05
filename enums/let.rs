fn main() {
   // let config_max = Some(3u8);
   // match config_max {
   //     Some(max) => println!("The max is {}", max),
   //     _ => (),
   // }
   
   let config_max = Some(3u8);
   if let Some(max) = config_max {
       println!("The max is {}", max);
   } // Use if let to take place of match
} 