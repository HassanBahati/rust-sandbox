//Arrays - fixed list where elements are the same data types

//import standard library
use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    //get single value
    println!("Single value {:?}", numbers[0]);

    //re-assign a value
    numbers[2] = 20;
    println!("{:?}", numbers);

    //get length 
    println!("Array length: {}", numbers.len());

    //arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    //standard library can be imported and mem accessed immediately
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers;

    //get all 
    println!("Slice: {:?}", slice);

    // get a range
    let slicedrange: &[i32] = &numbers[1..2];
    println!("Slice: {:?}", slicedrange);
}