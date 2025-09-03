#![allow(unused)]

fn take(s: String) {
    println!("{s}");
}
fn borrow(s: &String) {
    println!("{s}");
}
//Borrow - Temporarily use the value of a variable without taking ownership

fn main() {
    //Take Ownership
    let s = String::from("rust");
    // take(s);
    borrow(&s);
    println!("{s}");
    // println!("{s}"); //S is dropped here

    // - Create a reference (either mutable or immutable)
    //Immutable borrow  - s1,s2,s3 are immutable references
    let s = String::from("rust");
    let s1 = &s;
    let s2 = &s;
    let s3 = s2;
//Mutable borrow - s4 is a mutable reference
//  let mut s = String::from("rust");
//  let s1 = &mut s;
//  //S1 has read and write access to the value
//  s1.puah_str("🦀");

//  let s2 = &mut s;
//  s2.push_str("🦀");

    // - Does not take ownership
    // - Immutable by reference - any number of read-only access to the value
    // - Mutable  reference : only read and write access to the value at a time
    // Either immutable or mutable reference can be created, but not both at the same time ( not simultaneously)
let mut s = String::from("rust");
let s1 = &s;
let s2 = &s;

// let s3 = &mut s;
// println!("s1: {s1}");

    // - Reference must not outlive the value it references

    let s = String::from("rust");
    let s1 = &s;
    {
        let s2 = s;
    }
    //s2 and s no longer exists 
    //s1 is still reference to the value of s
    // println!("s1: {s1}");

    /* The problem: s1 is trying to reference s, but s has been moved and no longer exists. This creates a dangling reference - s1 is pointing to memory that has been freed.
Why this violates Rust's borrowing rules:
References must always point to valid data
A reference cannot outlive the data it's referencing
When s was moved to s2, the original s ceased to exist
s1 is now referencing invalid memory


= (move): Transfers ownership, invalidates the original
=& (borrow): Creates a reference, leaves ownership unchanged
Think of it like this:
Move: "I give you my car" → I no longer have a car
Borrow: "You can use my car" → I still have my car, you just have permission to use it

*/
//We can create multiple immutable references, but only one mutable reference
  
}
