//Loops - used to iterate until condition is met

pub fn run() {
    let mut _count = 0;

    //Infinite loop
    // loop {
    //     _count += 1;
    //     println!("Count: {}", _count);

    //     if _count == 20 {
    //         break;
    //     }
    // }

    //While loop
    // while _count <= 100 {
    //     if _count % 15 == 0 {
    //         println!("FizzBuzz")
    //     } else if _count % 3 == 0 {
    //         println!("Fizz")
    //     } else if _count % 5 == 0 {
    //         println!("Buzz")
    //     } else {
    //         println!("{}", _count);
    //     }

    //     _count += 1;
    // }

    //For range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz")
        } else if x % 3 == 0 {
            println!("Fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", x);
        }
    }
}
