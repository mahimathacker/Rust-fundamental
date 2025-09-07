#![allow(unused)]

#[derive(Debug)]

struct Point {
    x : f32,
    y: f32,
}
//Method is simply a function that is associated with a struct or a data type
impl Point {
    //Static Method

    fn new(x: f32, y: f32) -> Self { //Self is a keyword that refers to the struct itself
        Self { x, y }
    }

     fn move_to(&mut self,  x: f32, y: f32){
        self.x = x;
        self.y = y;
     }

}
fn main() {
//  let mut p = Point { x: 0.0, y: 1.0};
let mut p = Point::new(0.0, 0.0);
 p.move_to(1.0, 2.0);
 println!("{:?}", p);
}