/*
Primitive Types =
integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (numbers of bits they take in memory)
floats: f32, f64
boolean (bool)
characters (char)
tuples
arrays
*/

/*
Rust is a statically typed language, which means that it must know the types of all variables at the compile time, however, the compiler can usually infer what type we want to use based on the value and we use it
*/

pub fn run(){
    //By default this is "ui32"
    let x = 1;

    //by default this is "f64"
    let y = 2.5;

    //add explicit type
    let z: i64 = 473849384;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;

    //:? debug placeholder

    println!("{:?}", (x, y, z, is_active));

    //Get boolean from expression
    let is_greater = 10 > 5;

    println!("{:?}", (is_greater));

    //char type - only one character
    let a1 = "a";

    //can be a unicode/emoji
    let face = "\u{1F600}";

    println!("{:?}", (a1, face))
}