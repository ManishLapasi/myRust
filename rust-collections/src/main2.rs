fn main() {
    let data = "initial contents";
    let mut s = data.to_string();
    let s1 = "initial contents".to_string();
    let s2 = String::from("initial contents");
    println!("{}, {}, {}, {}", data, s, s1, s2);
    s.push_str(" some more content");
    s.push('!');
    println!("{s}");
    let s3 = s1 + &s; // s1 goes out of scope
    println!("{s3}");
    let s4 = format!("{s}-{s2}-{s3}");          // better way to concatenate strings!
    println!("{s4}");
}