#![allow(unused)]

fn main() {
    let x: Option<i32> = Some(5);
    match x {
        Some(val) => println!("option is {val}"),
        None => println!("option is None"),
    }
    let x: Option<i32> = Some(5);
    if let Some(val) = x {
        println!("x is {val}")
    };

    //Use if_let to match a value and assign it to a variable and if you do not want to match the none value then you can use the if_let syntax

    /* 
    In this specific match structure, we are primarily concerned with the Some(val) case. The None case is explicitly handled, but it results in no operation, which can feel like boilerplate.

Introducing if let for Enhanced Conciseness
For situations where you need to match against a single pattern and ignore all others, Rust provides the if let construct. It can be thought of as syntactic sugar for a match statement that only executes code for one specific pattern.
    */
}