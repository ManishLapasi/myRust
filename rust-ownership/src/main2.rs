fn main() {
    let s = String::from("hello world!");
    let r1 = &s[..5];
    println!("the first 5 characters are {r1}");
    let r2 = firstword(&s);
    println!("the first word is {r2}");
    //s.clear();                            // this will throw an error because clear() takes as its reference a mutable, which we canot do since we have an immutable too! (when we call firstword())

    let s_literal = "hello world!";
    let r1 = firstword(s_literal);
    println!("the first word is {r1}");
    let r1 = firstword(&s_literal[..]);
    println!("the first word is {r1}");
}

fn firstword(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item==b' ' {
            return &s[..i];
        }
    }
    return &s[..];

}