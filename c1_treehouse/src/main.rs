use std::io::stdin;

struct Visitor{
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self{
        // Koska tässä ei oo puolipistettä, se tarkoittaa "implicit return"
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string()
        }
    }

    fn greet(&self){
        println!("{}", self.greeting);
    }
}

fn ask_for_input() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_lowercase()
}

fn main() {
    let visitor_list = [
        Visitor::new("bob", "Waddup bob"),
        Visitor::new("ben", "Hey, Ben. How's it hanging?"),
    ];
    println!("Hello. What's your name?");

    let user_name = ask_for_input();

    println!("Hello {}", user_name);

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == user_name);

    match known_visitor {
        Some(visitor) => visitor.greet(),
        None => println!("Sorry. You are not in the visitors list."),
    }
}
