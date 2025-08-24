//Vector: collection of values of the same type: Grown and shrinkable at runtime vs array: fixed size at compile time

#![allow(unused)]

fn main() {
  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);

  println!("Vector: {:?}", v);

  let v = vec![1, 2, 3];
  println!("Vector: {:?}", v);

  let v = vec![1u8, 2, 3];

  let v = vec![0i8; 100];
  println!("Vector: {:?}", v);

  //Get value 
  println!("v: {}", v[1]);

//get is option<&i8> 
//Indexvalid => Some(&i8)
//Indexinvalid => None

println!("v[1]: {:?}", v.get(1000));

//Update 
let mut v: Vec<i8> = vec![1, 2, 3];
v[0] = 10;

//Pop - remove last element and return it
let mut v: Vec<i8> = vec![1, 2, 3];
let x: Option<i8> = v.pop();

println!("x: {:?}", x);

//Slice 
let v = vec![1, 2, 3, 4, 5];
let slice = &v[1..4];
println!("Slice: {:?}", slice);

}
