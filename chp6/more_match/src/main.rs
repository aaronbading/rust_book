#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn add_fancy_hat() {
    println!("Adding fancy hat!");
}
fn remove_fancy_hat() {
    println!("Removing fancy hat!");
}
fn move_player(num_spaces: u8) {
    println!("Moving player {} spaces!", num_spaces);
}
fn reroll() {
    println!("Rerolling the dice!");
}

fn main() {

    let value = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("value: {}", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    ///////////// Demonstrate Catch all other cases and use the value 
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    
    // Demonstrate catch all without using the value itself
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // Demonstrate catch all without running any code in that case 

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

}
