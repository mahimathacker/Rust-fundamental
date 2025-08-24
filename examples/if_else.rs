#![allow(unused)]

fn main() {
    let x: u32 = 10;

    let z: i32 = if x > 0 {
        println!("x is positive");
        1 //do not add return keyword as we are not returning  a value from main fun, we are returning from the if else block
    } else if x < 0 {
        println!("x is negative");
        -1
    } else {
        println!("x is zero");
        0
    };

    println!("z: {z}");
}
