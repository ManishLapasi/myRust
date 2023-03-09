fn main() {

    let s = String::from("hello");  // "hello" is allocated to heap, and s actually stores a pointer to this heap address
    takes_ownership(s);             // a new variable on stack is created that stores the pointer
    let x=5;
    makes_copy(x);
    println!("x is still {x}");
    //println!("but s is not {s} anymore!");     // this line would throw an error, because s has now gone out of scope!

    let s = String::from("world");
    takes_ownership(s.clone());                  // now we create a deep copy and pass it, so the original is still in scope!
    println!("but this new s is still in scope and has value: {s}");

    let s1 = String::from("newString");
    let s2 = s1;
    //println!("s1 is out of scope {s1}!")      // this line will throw an error too, since new assignment moves it out of scope!
    println!("s1 has been moved to s2, and the value of s2 is {s2}");

    let mut s3 = String::from("another string");
    s3 = returns_string(&mut s3);
    println!("did it return the value? it did!: {s3}! notice the change in the value returned!");

    let r1 = &s3;
    let r2 = &s3;
    println!("made two immutable pointers {} and {}",r1,r2);
    let mut r3 = &mut s3;
    r3.push_str("_not again!");
    println!("made a third immutable pointer {r3}");

    let x: usize = get_length(&s3);
    println!("length of the string is {x}");
}

fn takes_ownership(s: String) {
    println!("The value passed is {s}");
}       // the original value of s from main (or whatever function called this function) goes out of scope in main (or the caller)!

fn makes_copy(x: i32) {
    println!("The value passed is {x}");
}       // the original value in the caller does not go out of scope!

fn returns_string(s: &mut String) -> String {
    s.push_str("again!");
    println!("made a mutable pointer, the value is {s}"); // this is allowed because compiler recognises that r1 and r2 are out of scope!
    return (&s).to_string();
}

fn get_length(s: &String) -> usize {
    s.len()
}   // s does go out of scope! but it is the copied pointer that goes out of scope, not the original pointer! Thus, the
// original pointer still exists and can be referenced to in the caller i.e. main !!

// references are immutable by default!