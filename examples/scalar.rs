#![allow(unused)]

//Scalar  - data types that represent a single value

fn main() {
    //Signed int ; i8: 8-bit signed integer.

    // i16: 16-bit signed integer.

    // i32: 32-bit signed integer (this is the default integer type if not specified).

    // i64: 64-bit signed integer.

    // i128: 128-bit signed integer.

    // Range: -(2^(n-1)) to 2^(n-1) - 1
    let i0: i8 = -1; // Range: -128 to 127
    let i1: i16 = 2; // Range: -32,768 to 32,767
    let i2: i32 = 3; // Range: -2,147,483,648 to 2,147,483,647
    let i3: i64 = -4; // Range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    let i4: i128 = 5; // A very large range

    //Unsigned integers : Non negative numbers

    // Range: 0 to 2^n - 1
    let u0: u8 = 1; // Range: 0 to 255
    let u1: u16 = 2; // Range: 0 to 65,535
    let u2: u32 = 3; // Range: 0 to 4,294,967,295
    let u3: u64 = 4; // Range: 0 to 18,446,744,073,709,551,615
    let u4: u128 = 5; // A very large range, up to 2^128 - 1

    // Architecture-dependent integers
    let i5: isize = -6; // Will be i32 or i64
    let u5: usize = 6; // Will be u32 or u64

    // Floating point numbers
    let f0: f32 = 0.01;
    let f1: f64 = 0.02; // f64 is the default if not specified

    //Boolean
    let b: bool = true;
    let is_active: bool = false;

    //Characters
    let c: char = 'c';
    let z: char = 'ℤ';
    let heart: char = '❤';

    //Type conversion

    let i = -1;
    let u = i as u32;
    println!("{i} as u32 = {u}");

    //Min and Max numbers

    let i_max = i32::MAX;
    let u_max = u32::MIN;

    println!("{i_max}");
    println!("{u_max}");
}
