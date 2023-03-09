use std::io;


#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String)
}

#[derive(Debug)]
enum State {
    Georgia,
    Florida
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}


fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("We have a quarter from state {:#?}",state);
            25
        }
    }
}

fn plus_one (x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        None => None
    }
}

fn main() {

    let ip1 = IpAddrKind::V4(255,0,0,0);
    let ip2 = IpAddrKind::V6(String::from("::1"));

    println!("Values are {:#?}, {:#?}", ip1, ip2);

    println!("Hello, world!");

    let num1 = Some(5);
    let char1 = Some('e');

    println!("adding 1 to a Option var Some(5) gives: {:#?}", plus_one(num1));

    let not_a_num : Option<i32> = None;
    //println!("{}", plus_one(not_a_num));

    println!("{:#?}, {:#?}", num1, char1);

    let c1 = Coin::Quarter(State::Georgia);
    println!("Value of coin in cents: {}", value_in_cents(&c1));
    println!("Number of non quarters: {}", count_non_quarters(0, c1));

    let dice_roll: u32 = 9;
    roll_check(dice_roll);

}

fn roll_check(x: u32) {
    let mut dice_roll = x;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        _ => {
            let dice_roll = reroll();
            println!("dice_roll is now {dice_roll}");
            roll_check(dice_roll);
        }
    }
}

fn add_hat() {
    println!("hurray! you've added a hat!");
}

fn remove_hat() {
    println!("huzzah! you've removed a hat!");
}

fn reroll() -> u32 {
    let mut guess = String::new();
    println!("keep going, that value doesn't do anything, enter your roll!");
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line :(");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("that's not a number!");
            reroll()
        }
    };
    println!("you rolled {guess}");
    return guess;
}

fn count_non_quarters(count: u32, coin: Coin) -> u32 {
    if let Coin::Quarter(State) = coin {
        println!("It's a quarter from state {:?}", State);
        return count;
    } else {return count+1;}
}