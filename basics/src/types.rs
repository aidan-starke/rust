/*
    Primitive types--
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    (Number of bits they take in memory)
    (u = unsigned - no negative, i = signed - any)
    Floats: f32, f64
    Boolean: bool
    Characters: char
    Tuples (lists)
    Arrays (fixed length)
*/

/*
    Rust is a statically typed language, which means that it must know the types of all
    variables at the time of compile, however the compiler can usually infer what type
    we want to use based on the value and how we use it
*/

pub fn run() {
    //Default is i32
    let x = 1;

    //Default is f64
    let y = 2.5;

    //Add explicit type
    let z: i64 = 922337203685477;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active = true;

    //Get boolean from expression
    let is_greater = 10 > 5;

    //Char
    let a1 = 'a';
    let smiley = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, smiley));
}
