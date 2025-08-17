#![allow(unused)]

//String nd &str 
// Use String: when you need ownership ad mutability
// &str: read only string and string literals

fn main(){
    let msg: String = String::from("Hello Rust");
    let msg: String = "Hello Rust".to_string();

    let length: usize= msg.len();

    //String slice 
    let s: &str = &msg[0..5];
    println!("s = {}", s);

    //Conversion from string slice to string 

    let s: &str= "Hello World";
    let x : String = s.to_string();

    //Rust automatically converts &String into a &str

    let msg: String = String::from("Hello Rust");
    print(&msg);

    let s: &str= "Hello World";
    print(s);

    //Append  &str to String
    let mut msg: String = String::from("Hello Rust");
    msg += " Test";
    println!("{msg}");

//String interpolation - format!

let mut lang = "Rust";
let  emoji= "👾";
let  s = "Hello Rust 👾";
let mut s = "Hello".to_string();

s += " ";

s += lang;

s += " ";

s += emoji;

let s = format!("Hello {} {}", lang, emoji);




}

    fn print(s: &str){
        println!("{s}");
    }