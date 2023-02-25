use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "bahati";
    let status = "100%";

    println!("Commad: {}", command);

    if command == "hello" {
        println!("{}, how are you", name);

    }else if command == "status" {
        println!("Status is {}", status);

    }else{
        println!("that is not a valid command");
    }
}



//to run this file - cargo run hello