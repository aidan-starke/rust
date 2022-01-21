pub fn run() {
    //Print to console
    println!("Hello from the print.rs file");

    //Need string literal to format println
    println!("Number: {}", 1);

    //Basic formatting
    println!("{} is from {}", "Aidan", "Auckland");

    //Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Aidan", "Auckland", "Code"
    );

    //Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Aidan",
        activity = "hockey"
    );

    //Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    //Placeholder for debug traits
    println!("{:?}", (12, true, "Hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}
