/*
Primitive str = Immutable fixed-length string somewhere in memory
String = growable, heap-allocated data structure - use when you need to modify or own string data

**/


pub fn run(){
    let mut hello = String::from("Hello ");

    //Get length
    println!("Length: {}", hello.len());

    //add characters - characters require one quote
    hello.push('W');
    
    //Push a string 
    hello.push_str("orld!");

    //Capacity in bytes
    println!("capacity: {}", hello.capacity());

    //check if empty
    println!("Is Empty: {}", hello.is_empty());

    //check if contains
    println!("Contains 'World {}", hello.contains("World"));

    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // loop through trings by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }


    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing - only returns an error when not true
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    // println!("{}", hello);
    println!("{}", s);
}