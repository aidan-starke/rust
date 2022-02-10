//Structs - used to create custom data types

//Traditional struct
struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

//Tuple struct
struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    //Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //Name to tuple
    fn to_tuple(&self) -> (String, String) {
        (self.first_name.to_string(), self.last_name.to_string())
    }
}

pub fn run() {
    let mut c = Colour {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Colour: {} {} {}", c.red, c.green, c.blue);

    let mut _c = Color(255, 0, 0);

    _c.1 = 100;

    println!("Color: {} {} {}", _c.0, _c.1, _c.2);

    let mut p = Person::new("Jo", "Duthie");
    println!("Person: {} {}", p.first_name, p.last_name);

    println!("Full name: {}", p.full_name());

    p.set_last_name("Ardern");
    println!("New full name: {}", p.full_name());

    println!("Person tuple: {:?}", p.to_tuple())
}
