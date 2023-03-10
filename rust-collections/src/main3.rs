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

}