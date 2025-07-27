use std::io::stdin;

fn ask_for_input() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_lowercase()
}

fn main() {
    let visitor_list = ["bert", "bob", "jorma"];
    println!("Hello. What's your name?");

    let user_name = ask_for_input();

    println!("Hello {}", user_name);

    let mut allowed = false;
    for name in &visitor_list{
        if name == &user_name {
            allowed = true;
            break;
        }
    }

    if allowed {
        println!("Welcome to the Treehouse, {user_name}");
    }
    else{
        println!("Sorry. You cannot enter because your name is not in the visitors list");
    }
}
