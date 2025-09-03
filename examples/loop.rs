#![allow(unused)]

fn main() {
    let mut i = 0;
    loop {
        println!("looping");
        i += 1;
        if i > 5 {
            break;
        }
    }
    let mut i = 0;
    while i <= 5 {
        println!("while looping");
        i += 1;
    }

    for i in 0..6 {
        println!("for loop {i}");
    }
    let arr = [1, 2, 3, 4, 5];
    let n: usize = arr.len();
    for i in 0..n {
        println!("Arrray {i}");
        println!("Arrray {}", arr[i]);
    }
    for n in arr {
        println!("Arrray {n}");
    }

    let v = vec![10, 20, 30, 40, 50];

    for n in v.iter() {
        println!("vec {}", n);
    }

    for n in v.iter() {
        /*
        If you want to run the same for loop again then add iter as once this for loop
               for n in v {
            println!("vec {}", n);
            is ran then vector v is already used so cannot be used again and that's why we can not run it again. just use v.inter() and into_iter() to escape the ownership
          }

             */
        println!("vec {}", n);
    }

    let mut i = 0;
    let z: &str = loop {
        println!("looping");
        i += 1;
        if i > 5 {
            break "loop ends here";
        }
    };
    println!("Loop returns: {z}");
}
