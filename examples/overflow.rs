#![allow(unused)]

fn main() {
    let mut x = u32::MAX;
    x += 1;

    println!("u32 max: {}, x : {} ", u32::MAX, x);
    //After adding --release code won't panic here: cargo run --example overflow --release

    //u32 :: checked_add - return None on overflow

    let x = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", x); //Debug mode: return None

    //u32 :: wrapping_add - explicitly allow overflow
    let x = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {:?}", x)
}
