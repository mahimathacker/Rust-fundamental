#![allow(unused)]
 //Memory- stack and heap
    //Stack - Store the data fixed size,known at compile time,fast access,last in first out ,LIFO 

    //Heap - Store the data variable size,unknown at compile time,slow access,FIFO
    //Data is slower access than stack
    //Memory safety is enforced through rust's ownership and borrowing rules
//Stack datat type: i32,bool,char,tuple,array
//Heap datat type: String,Vec<T>,Hashmap<K,V>


fn main() {
   let x: i32 = 5;
   let arr: [i32: 3] = [1,2,3];

    //Heap 
    let mut s: String = "hello".to_string();
    s += "";

    let mut v = vec![];
    v.push(0);


    //if we use Box data type then i32 will be stored in the Heap

    let boxed = Box::new(1i32);
}