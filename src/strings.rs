// Primitive str = immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - for when you need to mod or own string data

pub fn run() {
    //str
    let hello = "Hello";

    //String
    let mut _hello = String::from("Hello ");

    //get length
    println!("Length: {}", hello.len());

    println!("{}", hello);

    //Push char
    _hello.push('W');

    println!("{}", _hello);

    //Push string
    _hello.push_str("orld");

    println!("{}", _hello);

    //Capacity in bytes
    println!("Capacity: {}", _hello.capacity());

    //Check if empty
    println!("Is empty: {}", _hello.is_empty());

    //Contains
    println!("Contains 'World': {}", _hello.contains("World"));

    //Replace
    println!("Replace: {}", _hello.replace("World", "There"));

    //Loop through string by whitespace
    for word in _hello.split_whitespace() {
        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion testing (no print on pass)
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
