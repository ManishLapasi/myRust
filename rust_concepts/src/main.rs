use std::io;

fn main() {
    println!("Hello, world!");
    let x = 5u8;        // initialise u8 with value 5
    println!("Initial value is {x}");
    let x: u8 = 5;      // same as before, just in a different way
    println!("Initial value is {x}");
    let x = x+1;
    {
        let x = x*2;
        println!("Value in this scope is {x}");
    }
    println!("Value after editing is {x}");

    let spaces = "  hello there!    ";
    println!("the value of spaces is {spaces}");
    let spaces = spaces.len();
    println!("the value of spaces is {spaces}");

    const VAR: f32= 1000.0;
    println!("The value of const is {VAR}, you can never edit this value");

    let t = true;
    let f: bool = false;
    println!("The booleans are {t} and {f}");

    let tup: (u8, f32) = (5,412.21);
    let (x,y) = tup;
    println!("The tuple is {x}, {y}");
    let x = tup.0;
    let y = tup.1;
    println!("The tuple is {x}, {y}");

    let (add, subtract, multiply, divide, remainder) = (5.0+3.5, 5.0-3.5, 5.0*3.5, 5.0/3.5, 5%3);
    println!("The arithmetic ops values are {add}, {subtract}, {multiply}, {divide}, {remainder}");

    let a = [1,2,3,4,5];
    let (x,y) = (a[0],a[1]);
    println!("The first two values of the array are {x},{y}");

    loop {
        println!("Enter an array index: ");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number, let's try again :D");
                continue;
            }
        };
        let element = a[index];
        println!("the element at index {index} is {element}");
        break;
    }
}
