#![allow(unused)]
#[derive(Debug)]

enum MathError {
    DivideByZero,
    Other,
}

/*
Error Handling!

panic!:

Use When: Unrecoverable errors, typically bugs in logic where the program's state is invalid and continuing execution is unsafe or nonsensical. Examples include invariant violations or critical failures during initialization.

Effect: Crashes the current thread (and usually the program).

Option<T>:

Use When: A value might be present or absent, and absence is a normal, expected possibility rather than a true "error."

Represents: Some(T) (value present) or None (value absent).

Examples: Finding an item in a collection (Vec::get, HashMap::get), optional function arguments, or fields in a struct that may not always be set.

Result<T, E>:

Use When: An operation can fail, and you need to communicate details about the failure. This is the most common way to handle recoverable errors.

Represents: Ok(T) (operation succeeded with value T) or Err(E) (operation failed with error E).

Advantages:

Expressiveness: Clearly distinguishes success from failure and provides an error value E for context.

Recoverability: Allows calling code to inspect the error and decide how to proceed (e.g., retry, log, return a default).

Type Safety: Using custom enums for E (like MathError) makes error handling more specific and robust than using simple strings. The compiler helps ensure all error variants are considered.
*/
fn main() {
    //panic mac- crash the program
    // panic!("crash the program");

    let v = vec![1, 2, 3];
    // // //v[99]; : Index out of bounds error

    let x: Option<&i32> = v.get(1);
    match x {
        Some(val) => println!("The 1th element is: {:?}", val),
        None => println!("Element at index 1 is: None"), // Output: Element at index 99 is: None
    }

    //Result<T,E> : Success or Failure = Ok(T) | Err(E)

    let x = 1;
    let y = 0;

    let q: Result<i32, MathError> = if y != 0 {
        Ok(x / 0)
    } else {
        Err(MathError::DivideByZero)
    };

    match q {
        Ok(val) => println!("value is = {:?}", val),
        Err(err) => println!("error is = {:?}", err),
    }
}
