#![allow(unused)]

#[derive(Debug)] //Add this macro to make struct work
//Struct : Groups different data types in one data type (Place)

struct Point {
    x: i32,
    y: i32,
}

struct Point3D(i32, i32, i32); //represent in tuple
struct Empty; //empty struct

#[derive(Debug)]
struct Circle {
    //Nested struct
    radius: i32,
    center: Point,
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("{:?}", p);
    println!("x: {}, y: {}", p.x, p.y);

    let p = Point3D(10, 20, 30);
    println!("Points: {}, {}, {}", p.0, p.1, p.2);

    let empty = Empty;

    let circle = Circle {
        radius: 10,
        center: Point { x: 10, y: 20 },
    };
    println!("Circle: {:#?}", circle);

    //ShorCut
    let x: i32 = 10;
    let y: i32 = 20;
    let p = Point { x: x, y: y };

    let p = Point { x, y };

    //Copy fields

    let p0 = Point { x: 10, y: 20 };
    let p1 = Point { x: 1, y: p0.y };
    let p1 = Point { x: 2, ..p0 };
    println!("Point1 Copy: {:?}", p1);

    //Update
    let mut p = Point { x: 10, y: 20 };
    p.x += 1;
    p.y = 99;
    println!("Updates: {:?}", p);
}
