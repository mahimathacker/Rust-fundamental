pub mod a;

pub fn print() { //public by adding pub keyword
    println!("Hello from public print");
    
    }
    
fn private_print() { //private by default
        a::print();
        println!("Hello from print");
    
    }