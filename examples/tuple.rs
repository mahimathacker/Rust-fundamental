#![allow(unused)]

//Tuples - fixed size, mixed types, known at compile time 

fn return_many() -> (u32, bool) {
    (1u32, true)
}

fn no_return() {}
fn main() {

    let t: (bool, char, u32) = (true, 'a', 1);
    println!("{}, {}, {}", t.0, t.1, t.2);

    //Empty Tuple = Unit Tuple 
    //Result<(), String> = Ok(()) | Err ("")
    let t = ();

    //Nested tuple 
let nested = (('a', 1.23), (true, 1u32, -1i32), ());
println!("nested.0.1: {}", (nested.0).1 );

//descructing tuple

    let t: (bool, char, u32) = (true, 'a', 1);

let (a, b, c) = t;
println!("a = {} , b = {}, c = {}", a, b, c);

//partial desctructing (ignore first and last values)

let (_, b, _)  = t;

//function that return multiple values using a tuple 
let (a,b)  = return_many();

}