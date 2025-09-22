#![allow(unused)]
use std::cmp::PartialOrd;

//Trait bounds - partial order helps to compare two values
//PartialOrd is a trait that allows us to compare two values

fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y { x } else { y }
}

trait A {}
trait B {}
trait C {}

impl A for i32 {}
impl B for i32 {}
impl C for i32 {}

fn a<T: A>(x: T) {}
fn ab<T: A + B>(x: T) {}
//fn w<T: A + B, U: B  + C> ( x: T, y: U) {}
fn w<T, U>(x: T, y: U)
where
    T: A + B,
    U: B + C,
{
}

fn main() {
    let u: u32 = 1;
    a(u);
}
