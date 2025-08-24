#![allow(unused)]

#[derive(Debug, PartialEq)] //Add this macro to make enum and struct work
//enums expresses values that the data type can take on 

enum Command {
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize { width: u32, height: u32}
}

//main
fn main() {

    let cmd: Command = Command::Play;
    let cmd: Command = Command::Skip(10);
    let cmd: Command = Command::Resize {width: 100, height: 120};
// DEBUG
    println!("{:?}", cmd); //:? to work with debug

    //compare 2 values 
    //PartialEq
    let cmd0: Command = Command::Play;
    let cmd1: Command = Command::Skip(10);

    println!("{}", cmd0 == cmd1);

    //Option,<T> = Some(T) | None: Indicates presence of the value or absense in the value(None)
    let x: Option<i32> = Some(1);
    let x: Option<i32> = None;


    //Result<T, E> = Ok(T) | Err(E)

    let x: Result<i32, String> = Ok(100);

    let x: Result<i32, String> = Err("Not a number");

    
}