#[allow(unused)]
use std::collections::HashMap;
//Common functions call: maps, filters, collect
//Iterators: 
//two loops won't run if v is used another time in the same scope hence we need to use iter() or into_iter() to make it work
fn main() {
    let vals : Vec<u32> = vec![1,2, 3, 4, 5];
//into_iter() : takes ownership of the vector and returns an iterator: iterator T 
//iter() : borrows the vector and returns an iterator: iterator &T : we can twice with iter for same loop name
//iter_mut() : borrows the vector and returns an iterator: iterator &mut T

    // for i: u32 in v.into_iter() {

    // }

    // for i: u32 in v.iter() {

    // }

    //map : f (x: &u32) -> u32
    //collect() is what consumes the iterator and turns it into a collection (like a Vec):

 let new_vals: Vec<u32> =vals.iter().map( |x: &u32| x + 1).collect(); //get &T here T his &u32 so we can add 1 to it
    println!("v2: {:?}", new_vals);
     //Another data type 

 let vals1: Vec<(&str, u32)> = vec![("a", 1), ("b", 2), ("c", 3)];

 let v3: Vec<(String, u32)> = 
 vals1.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();    

 println!("vals: {:?}", v3);

 let v4: HashMap<String, u32> = 
 vals1.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();

 println!("hashmap: {:?}", v4);

 //Chaining Filter and Map
 let vals : Vec<u32> = vec![1,2, 3, 4, 5];

 let filter_vals: Vec<u32> = vals.iter().filter(|x: &&u32 | **x <= 3).map(|x : &u32| x + 1).collect(); //** dereference the reference to get the value and && first fives refrence of the u32 from the iterator and for filter
 println!("filter_vals: {:?}", filter_vals);

 }


