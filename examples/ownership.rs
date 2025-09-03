#![allow(unused)]

//Owneship rules
fn take(s: String) {
    println!("{s}");
    //S is dropped he
}
fn copy(i: i32) {
    println!("{i}");
    //I is copied here
}

fn main() {
    //1. Each value in Rust has an owner
    // Owner of s is s
    let s = String::from("rust");

    // Owner of x is x
    let x = 5;

    //2. There can only be one owner at a time

    let s = String::from("dog");
    // Owner of s is s1
    let s1 = s;
    // println!("{s1}");

    // println!("{s}")
    //3. When the owner goes out of scope, the value will be dropped

    let s = String::from("rust");
    {
        s;
        //Scope of s ends here as s is dropped
    }
    // println!("{s}"); //outside of the scope we can no longer access the value of s

    let s = String::from("rust");
    // take(s);
    // println!("{s}");

    //Ownership does not move for that type it is a scalar type
    let i = 5;
    let i1 = i;
    let i2 = i1;

    let y = x;
    println!("x: {x}, y: {y}");

    copy(i);
    println!("i: {i}");
}
