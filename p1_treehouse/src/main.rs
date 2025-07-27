use std::io::stdin;

#[derive(Debug)]
struct Visitor{
    name: String,
    action: VisitorAction,
    age: u8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: u8) -> Self{
        // Koska tässä ei oo puolipistettä, se tarkoittaa "implicit return"
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet(&self){
        match &self.action{
            VisitorAction::Accept => println!("Welcome to the Treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the Treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 18 {
                    println!("Do not serve alchohol to {}.", self.name);
                }
            },
            VisitorAction::Probation => {
                println!("{} is now a probationary member", self.name);
            },
            VisitorAction::Refuse => {
                println!("You are not allowed to enter the Treehouse, {}", self.name)
            },
        }
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn ask_for_input() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bob",VisitorAction::Accept, 45),
        Visitor::new("ben", VisitorAction::AcceptWithNote { 
            note: String::from("Lactose-free milk is in the fridge")
        }, 15),
        Visitor::new("Fred", VisitorAction::Refuse, 30),
    ];

    loop {
        println!("Hello. What's your name? (Leave empty and press ENTER to quit)");

        let user_name = ask_for_input();

        if user_name.is_empty(){
            break;
        }

        println!("Hello {}\n", user_name);

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == user_name);

        match known_visitor {
            Some(visitor) => visitor.greet(),
            None => {
                println!("You are not on the visitors list, so we'll add you there.");
                println!("Since the Treehouse serves alchohol, we need to ask for your age. How old are you?");
                let age:u8;
                loop {
                    let age_input = ask_for_input();
                    age = match age_input.parse() {
                        Ok(age) => age,
                        Err(_) => {
                            println!("Please enter a valid age");
                            continue;
                        },
                    };

                    break;
                }
                visitor_list.push(Visitor::new(&user_name, VisitorAction::Probation, age));
            },
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
