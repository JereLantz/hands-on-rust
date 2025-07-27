use std::io::stdin;

#[derive(Debug)]
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
    let mut visitor_list = vec![
        Visitor::new("bob", "Waddup bob"),
        Visitor::new("ben", "Hey, Ben. How's it hanging?"),
    ];

    loop {
        println!("Hello. What's your name? (Leave empty and press ENTER to quit)");

        let user_name = ask_for_input();

        if user_name.is_empty(){
            break;
        }

        println!("Hello {}", user_name);

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == user_name);

        match known_visitor {
            Some(visitor) => visitor.greet(),
            None => {
                println!("You are not on the visitors list, so we'll add you there.");
                println!("how should we greet you in the future?");
                let greeting = ask_for_input();
                visitor_list.push(Visitor::new(&user_name, &greeting));
            },
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
