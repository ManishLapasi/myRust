use std::io;
use std::cmp::Ordering;

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

    let x: i32 = f2(5, 'h', true);
    println!("The returned value is {x}");
    let y: u32 = f3(10);

    let z: u32 = f4(5);
    println!("the value of z is {z}");

}

fn f2(x: i32, y: char, t: bool) -> i32{
    println!("This is a custom function! You passed the value {x} and char {y} to this function.");

    // let's go over statements and expressions!

    let p = {                                           // This is the start of an expression
        let q = 7;                                      // This is a statement that sets q=7 
        println!("The value of q is {q}");
        q+1
    };                                                  // This is the end of the expression that evaluates to q+1=8
    println!("The value of p is {p}");

    let p = if t {2*p} else {4*p};
    println!("The value of p is now {p}");

    if t {
        return p+x;                                     // you use ; here since it is an expression that is evaluated and returned
    }
    
    p-x                                                 // no ; here since it is just a value!
}


fn f3(count: u32) -> u32{
    // let's write a loop!

    let mut counter: u32 = 0;

    // standard loops 

    let y = loop {
        counter += 1;
        if counter==count {
            break 2*counter;
        }
    };
    println!("The loop ran {counter} times and will return {y}");
 
    // while loops

    while counter>0{
        counter -= 1;
    }
    println!("did a while loop till counter reached {counter}");

    // for loops

    let a = [1,2,3,3,6];
    for element in a {
        println!("the value of element is {element}");
    }

    // rev constructor!

    for element in (1..6).rev(){
        println!("the value of element is {element}");
    }

    y

}


fn f4(count: u32) -> u32{
    // loop inside a loop time!

    let mut counter: u32 = 10;
    let x: u32 = 'outerloop: loop {
        println!("counter is now {counter}");
        let mut innercounter = counter;
        loop {
            if innercounter<count{
                println!("breaking outer loop at value {innercounter}");
                break 'outerloop counter;
            }

            if innercounter<counter-2{
                println!("breaking inner loop at value {innercounter}");
                break;
            }
            innercounter -= 1;
        };
        counter -= 2;
        if counter<count {
            break counter;
        }

    };
    x
}
