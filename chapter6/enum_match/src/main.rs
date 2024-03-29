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

fn main() {
    let value = value_in_cents(Coin::Penny);
    println!("coin value is {}", value);

    let value2 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("coin value is {}", value2);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player(),
    }
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

fn add_fancy_hat() {
    println!("add_fancy_hat");
}

fn remove_fancy_hat() {
    println!("remove_fancy_hat");
}

fn move_player() {
    println!("move_player");
}
