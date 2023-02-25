//vectors - resizable arrays

//import standard library
use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers);

    //get single value
    println!("Single value {:?}", numbers[0]);

    //re-assign a value
    numbers[2] = 20;
    println!("{:?}", numbers);

    //add to vector
    numbers.push(5);
    numbers.push(6);

    //pop off last value
    numbers.pop();

    //get length 
    println!("vector length: {}", numbers.len());

    //vectors are stack allocated
    println!("vector occupies {} bytes", std::mem::size_of_val(&numbers));

    //standard library can be imported and mem accessed immediately
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers;

    //get all 
    println!("Slice: {:?}", slice);

    // get a range
    let slicedrange: &[i32] = &numbers[1..2];
    println!("Slice: {:?}", slicedrange);

    //loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }
        println!("Numbers vec: {:?}",numbers);
  
}