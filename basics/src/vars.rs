//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run() {
    let name = "Aidan";
    //need mut to be able to change var
    let mut age = 26;
    println!("My name is {} and I am {}", name, age);

    age = 27;
    println!("My name is {} and I am {}", name, age);

    //Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assing multiple variables
    let (my_name, my_age) = ("Aidan", 26);
    println!("{} is {}", my_name, my_age);
}
