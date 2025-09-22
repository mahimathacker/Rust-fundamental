#![allow(unused)]
#[derive(Debug)]

//Lifetime - The period of time for which a reference is valid, everything in rust has a lifetime
//So to provide refrence is valid both X and Y must be valid and we add 'a here to make it valid for both X and Y
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn print_refs<'a, 'b>(x: &'a str, y: &'b str) {
    println!("x: {x}, y: {y}");
}

struct Book<'a> {
    title: &'a str,
}

impl<'a> Book<'a> {
    fn edit(&mut self, new_title: &'a str) {
        self.title = new_title;
    }
}

fn main() {
    let x = "Hello".to_string();
    let y = "Rust intro".to_string();
    //  let z = {
    //     let y = "Rust intro".to_string();
    //     longest_str(&x, &y)
    //     //here y is dropped so z won't refer to y and give an error
    //      };
    let z = longest_str(&x, &y);

    println!("Longest string is { :? }", z);

    //static lifetime
    let s: &'static str = "rust".to_string(); //It will live for the entire program

    //Placeholder lifetime
    //  let p: &'_str = "rust".to_string(); //_ is a placeholder for the lifetime and it is used to infer or modufy the lifetime of the variable
}
