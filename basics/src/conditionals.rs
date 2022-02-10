//Conditionals - used to check the condition of something and act on the result

pub fn run() {
    let age = 17;
    let check_id = false;
    let knows_person = true;

    //If/Else
    if age >= 18 && check_id || knows_person {
        println!("Bartender: What would you like to drink?");
    } else if age < 18 && check_id {
        println!("Bartender: Sorry you have to leave");
    } else {
        println!("Bartender: I'll need to see your id");
    }

    //Shorthand if
    let is_of_age = if age >= 18 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
