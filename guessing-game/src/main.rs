use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=500);

    loop {
        println!("Pick a number:");

        let mut guess = String::new(); /* This creates a "mutable" variable with name "guess" and "binds it (=)" 
            to a "new" instance of "String", which we created by accessing the function "new()"
            that returns a new instance of String */

        io::stdin()                    // use "io" package that you imported
            .read_line(&mut guess)     // read line into variable referenced by "guess"
            .expect("Failed to read line :(");      // what to do if it fails

        println!("Your guess: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number, let's try again :D");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("That's correct, you win!");
                break;
            }
        };
    }
}
