enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other), // other is the value of dice_roll
        // _ => reroll(), // _ is a catchall, it will match anything but not bind to a value
        _ => (), //nothing happens
    }

    let opt: Option<String> = Some("Hello, World!".to_string());

    // match opt {
    //     Some(s) => println!("Some: {}", s),
    //     None => println!("Nothing"),
    // }

    //borrow matching!
    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("Nothing"),
    }

    println!("{:?}", opt);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(State) => {
            println!("State quarter from {:?}!", State);
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

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
fn move_player(num_spaces: u8) {}
