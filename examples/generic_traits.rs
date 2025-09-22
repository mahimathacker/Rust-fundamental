#![allow(unused)]

trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

impl List<u32> for (u32, bool, char) {
    fn count(&self) -> usize {
        3
    }

    fn first(&self) -> &u32 {
        &self.0
    }
}

impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }

    fn first(&self) -> &T {
        &self[0]
    }
}

fn main() {
    let t = (1u32, true, 'a');
    let v = vec![1u32, 2, 3];

    println!("Count: {}", t.count());
    println!("First: {}", t.first());
    println!("Count: {}", v.count());
    println!("First: {}", v.first());
}
