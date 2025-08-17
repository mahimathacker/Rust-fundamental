#![allow(unused)]

//Constants

const NUM: u32 = 1;
fn main() {
    //Variables
    // - Immutable by default
    // - Use mut keyword to make it mutable

    //let  x = 1;
    //x += 1; //Wrong as it is immutable
    let mut x = 1;
    x += 1;

    //Type interface: Int have type i32 in rust
    let y: i32 = -1;
    let z = -1;

    //Shadowing - Type change for the same variable

    let x: i32 = 1;
    let x: i32 = 2;
    let x: bool = true;

    //TYPE placeholder
    let x: _ = 1234; //Let the rust compiler figured out the value of the datatype of 

    //println!

    let x = 1;
    println!("X is {}", x);

    //Inline
    println!("X is {x}");

    //Positional Arguments

    // let z: i32  = x + x;
    // println!("{x} + {x} = {z}")

    println!("{0} + {0} = {1}", x, x + x);

    //Debugging feature
    println!("DEBUG: x {:?}", x);
    println!("DEBUG: x {:#?}", x);
}
