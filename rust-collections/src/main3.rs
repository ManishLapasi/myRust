use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(1,1);
    scores.insert(2,4);

    for (key,value) in &scores {
        println!("{key}: {value}");
    }

    let s = scores.get(&2).copied().unwrap_or(0);
    println!("{s}");

    let s = scores.entry(3).or_insert(50);
    println!("{:?}", scores);


    let text = "hello world it's a wonderful world";

    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}