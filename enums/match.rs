#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main () {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);

    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat();
    //     7 => remove_fancy_hat();
    //     _ => reroll();              // Use _ to catch any value
    // }
    
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat();
        7 => remove_fancy_hat();
        other => move_player(other); // Bind value to other then run right side of arm
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}