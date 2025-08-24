#![allow(unused)]
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("black".to_string(), 10);
    println!("scores: {:?}", scores);

    //get
    let score: Option<&u32> = scores.get("black");
    println!("score: {:?}", score);

    //Update
    let score: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *score += 1;
    println!("Blue score: {:?}", scores.get("blue"));
}
