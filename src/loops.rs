// Loops - used to iterate untila condition is met 

pub fn run(){
    let mut count = 0;

    //infinite loop
    // loop{
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    //while loop (fizzbuzzz)
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("fizzbuzzz");
    //     }else if count % 3 == 0 {
    //         println!("fizz");
    //     }else if count % 5 == 0{
    //         println!("buzz");
    //     }else{
    //         println!("{}", count);
    //     }

    //     //increament
    //     count+=1;
    // }

    //for range loop
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzzz");
        }else if x % 3 == 0 {
            println!("fizz");
        }else if x % 5 == 0{
            println!("buzz");
        }else{
            println!("{}", x);
        }
    }
}