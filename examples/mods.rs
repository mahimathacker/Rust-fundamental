#![allow(unused)]
use hello_rust::my;
// use hello_rust::{my, foo};
//Modules are a way to organise a code in rust

// mod foo {
//     pub fn print() {
//         println!("Hello from foo");
//     }
// }

// mod my {
// //    use super::foo; //Look for the module one above the current module or parent module
// //    //foo
// //    // - my
// //    // a

// //    pub fn print_foo() {
// //     foo::print();
// //    }

//    pub fn print() { //public by adding pub keyword
//         println!("Hello from public print");

//     }
//     fn private_print() { //private by default
//         a::print();
//         println!("Hello from print");

//     }
//    pub mod a { //public by adding pub keyword to access it from the outside
//         use super::super::foo;

//         pub fn print_foo() {
//             foo::print();
//         }
//         pub fn print() {
//             println!("a");
//         }
//        pub struct S {
//             pub id: u32,
//             pub name: String
//         }

//         pub fn build(id: u32) -> S {
//             S {
//                 id,
//                 name: " ".to_string()
//             }
//         }
//     }
// }

fn main() {
    my::print();
    my::a::print();
    // let s = my::a::S {
    //     id: 0,
    //     name: "rust".to_string()
    // };
    let s = my::a::build(0);
    my::a::print_foo();
}
