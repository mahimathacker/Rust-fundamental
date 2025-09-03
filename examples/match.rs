#![allow(unused)]

//match : allows us to compare values and execute the code depending on the value that was being matched

fn main() {
    let x: i32 = 4;
    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        3 => println!("x is not 1 or 2"),
        _ => println!("x is not 1, 2 or 3"),
    }
    match x {
        1 | 2 | 3 | 4 | 5 => println!("x is 1,2,3,4 or 5"),
        _ => println!("x is not 1, 2, 3, 4 or 5"),
    }
    let x = 10;
    match x {
        //Range match
        1..=10 => println!("x is between 1 and 10"), //including 10
        _ => println!("x is not between 1 and 10"),
    }
    match x {
        //Range match  value returned from the match is stored in the variable i
        i @ 1..=10 => println!("x is between 1 and 10 and i is {i}"), //including 10
        _ => println!("x is not between 1 and 10"),
    }

    let x: Option<i32> = Some(10);
    match x {
        Some(val) => println!(" x is {val}"),
        None => println!("x is None"),
    }

    let res: Result<i32, String> = Ok(10);
    match res {
        Ok(val) => println!(" x is {val}"),
        Err(err) => println!("x is {err}"),
    }

    let x: Option<i32> = Some(9);
    let z: i32 = match x {
        Some(val) => val,
        None => 0,
    };
    println!("z: {z}");
}
