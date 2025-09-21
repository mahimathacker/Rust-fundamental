#![allow(unused)]
#[tokio::main]

use std::time::Duration;
use tokio:: { join, select};

//Join mactro : executes multiple features concurrently
// - waits for all the features to complete
// - returns a tuple of the results

//Select - executes multiple features concurrently
// - returns as soon as one of the features completes
// - cancelled the rest features (drops them)

async fn make(val: &'static str, dt: u64) -> &'static str { 
    /*The ' (single quote) is Rust's syntax for declaring and using lifetimes. It's not optional 
    it's part of the language syntax. 
    & = reference (borrowed value)
'static = lifetime name (the ' is required)
str = string slice type*/

//:? : Debug print

 tokio::time::sleep(Duration::from_millis(dt)).await;
 val
}

async fn main() {
 let(rest1, rest2, rest3) = join!(
    make("hello", 100),
    make("Orange", 50),
    make("Apple", 30)
 )
println!("join: rest1 = {:?}", rest1);
println!("join: rest2 = {:?}", rest2);
println!("join: rest3 = {:?}", rest3);

}