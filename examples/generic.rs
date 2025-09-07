#![allow(unused)]

//Generic type - these are parameterized types : meaning they can be used with different types
//useful when we want to write code that can be used with different types
//Result, Option, Vec

// enum Option<T> { //T works as a placeholder for the type 
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
struct Point<T> {
    x: T,
    y: T,
}

fn swap<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
} 

fn main() 
{
    let v: Vec<i32> = vec![1i32,2,3];
    let p: Point<i32> = Point { x: 1, y: 2 };
let  a : u32 = 1;
let  b : i32 = 2;
    let (a, b) = swap(a, b);
    // println!("a: {a}, b: {b}");
}
