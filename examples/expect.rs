#![allow(unused)]

fn main() {
    let x: Option<i32> = Some(1);
    let v: i32  = match x {
        Some(val) => val,
        None => panic!("No value found")
    };

    // unwrap - unwraps the value of the option or panics if the value is none
    let i = x.unwrap();
    println!("i: {i} ");

    let x: Result<i32, String> = Ok(3);
    let v: i32  = match x {
        Ok(val) => val,
        Err(err) => panic!("No value found")
    };

    // let i = x.unwrap();
    // println!("Result: {i}");

    // let x: Result<i32, String> = Err("error".to_string());

    // let i = x.unwrap();
    // println!("error: {i}");

    //Expect behaves like unwrap but allows you to provide a custom error message
    let x: Result<i32, String> = Err("something went wrong".to_string());
    x.expect("something went wrong");

    /* 
    unwrap() vs expect()
Both methods do the same thing functionally - they extract the value from Option<T> or Result<T, E> and panic if there's no value. The difference is in the error message they provide when they panic.
unwrap()
Generic panic message: Always shows the same generic message
For Option: "called Option::unwrap() on a None value"
For Result: "called Result::unwrap() on an Err value"
expect()
Custom panic message: You provide a meaningful error message
Better debugging: Helps you understand WHY the panic occurred
    */
}