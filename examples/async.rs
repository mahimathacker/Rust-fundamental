#![allow(unused)]
/*
Attributes in Rust must be attached to specific items, not just "declared at the top" like variables or imports.

// On functions
#[tokio::main]
async fn main() { }

// On structs
#[derive(Debug)]
struct MyStruct;


*/
struct Tomoto;
struct Lettuce;
struct Cheese;
struct Pattry;
struct Bun;

struct Hamburger {
    pub tomoto: Tomoto,
    pub lettuce: Lettuce,
    pub cheese: Cheese,
    pub pattry: Pattry,
    pub bun: Bun,
}

async fn toast_bun() -> Bun {
    Bun
}

async fn cook_pattry() -> Pattry {
    Pattry
}

async fn get_veggies() -> (Tomoto, Lettuce) {
    (Tomoto, Lettuce)
}

async fn get_cheese() -> Cheese {
    Cheese
}

async fn make_hamburger_seq() -> Hamburger {
    let bun = toast_bun().await;
    let pattry = cook_pattry().await;
    let (tomoto, lettuce) = get_veggies().await;
    let cheese = get_cheese().await;
    println!("Hamburger made");

    Hamburger {
        bun,
        pattry,
        tomoto,
        lettuce,
        cheese,
    }
}

async fn make_hamburger() -> Hamburger {
    /*
    tokio::join! runs futures concurrently and waits for all to complete.
    You pass the futures themselves, and tokio::join! handles the awaiting internally.
     The .await is only used when you want to await a single future sequentially.
    */
    let (bun, pattry, (tomoto, lettuce), cheese) =
        tokio::join!(toast_bun(), cook_pattry(), get_veggies(), get_cheese(),);

    println!("Hamburger made");

    Hamburger {
        bun,
        pattry,
        tomoto,
        lettuce,
        cheese,
    }
}

#[tokio::main]
async fn main() {
    make_hamburger().await;

    let future = make_hamburger(); //This is a future that will be executed when the make_hamburger function is called and if we add.wait it will run
    future.await;

    //What's the difference between threads vs async: Both works same but async is more efficient and faster than threads?
    //spawing a thread will creash this program because of its limit reach

    //Memory limit swawn won't be an issue with async and await

    let mut handles = vec![];

    // for i in 0..10000000{
    //     handles.push(std::thread::spawn( move ||{
    //         std::thread::sleep(std::time::Duration::from_millis(1000));
    //         println!("Thread {i} finished");
    //     }))
    // }

    //Need to parallisng comutational: use threads
    //Need to paralling waiting time: use async awaits
    for i in 0..1000000 {
        let fut = async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            println!("Hamburger {i} finished");
        };

        let handler: tokio::task::JoinHandle<()> = tokio::task::spawn(fut);
        handles.push(handler);
    }

    for handle in handles {
        // handle.join().unwrap();
        handle.await.unwrap();
    }
}
