use restaurant::eat_at_restaurant;
use std::io;

fn main() {
    println!("hi! what type of bread would you like with your breakfast?");
    let mut guess = String::new();
    io::stdin().
        read_line(&mut guess).
        expect("Sorry, that isn't a valid type of bread");
    let guess = guess.trim();
    eat_at_restaurant(&guess);
    println!("have a great day!");
}