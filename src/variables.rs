// Variables are immutable by default

// Varibales hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    //we use let to create variable
    let name = "Bahati";
    let mut age = 34;

    println!("My name is {} and i am {}", name, age);

    // variables can be reassigned by adding mut to the variable
    age = 60;

    println!("My name is {} and i am {}", name, age);

    //Define constant - data type had to be defined
    const ID: i32 = 001;

    println!("ID: {}", ID);


    //Assign multiple variables at once
    let (my_name, my_age) = ("Bahati", 12);

    println!("{} is {}", my_name, my_age)
}