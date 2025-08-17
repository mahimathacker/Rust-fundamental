#![allow(unused)]

fn add_with_return(x: u32, y: u32) -> u32 {
    return x + y;
}

fn add_without_return(x: u32, y: u32) -> u32 {
    println!("x: {x}");
    println!("y: {y}");
    x + y
}

// fn add1( x: u32, y: u32) -> (u32, bool){
//  return (x + y, true);
// }

fn print(s: String) {
    println!(" {s} {s} {s} {s} {s} ");
}
fn main() {
    let x = 1;
    let y = 1;

    let z = add_without_return(x, y);
    println!(" {x}+ {y} = {z}");
    print("🦚".to_string());
}
