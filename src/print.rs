// create a function and run it in main.rs
pub fn run(){
    // Print to console
    println!("Hello from the print.rs file");

    // placeholders are needed for anything to be replaced 
    println!("Number: {}", 1);
    //multiple placeholders 
    println!("{} is from {}", "Hassan", "Uganda");


    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Bahati", "Uganda", "Code");

    // Named Arguments 
    println!("{name} likes to play {activity}", name = "bahati", activity = "Tennis");

    //PLaceholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}