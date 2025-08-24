#![allow(unused)]

//Array = Colllection of elements ith length known at compile time
//Slice = Length not known at compile time

fn main() {
    //Array
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[0]: {}", arr[0]);

    //Write

    //arr[1] = 99;

    let arr: [u32; 10] = [0; 10]; //print 10 0s 
    println!("arr: {:?}", arr);

    //Slice
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];

    //First 3
    let s: &[i32] = &nums[..3];

    //Last 3 indexes : 7,8,9
    let s: &[i32] = &nums[7..];

    //Middle 4
    let s: &[i32] = &nums[3..7];
    println!("mid 4 {:?}", s);
}
