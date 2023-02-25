// reference pointers - point to a resource in memory

pub fn run(){
    //primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    //With non-primities, if oyu assign another varaible to a piece of data, the first varaible will no longer hold that value. You will need to use a reference (&) to point to the resource

    //vectors
    let vec1 = vec![1,2,3];
    // for non-primitive create a reference 
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}
