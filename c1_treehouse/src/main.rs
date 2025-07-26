use std::io::stdin;

fn ask_for_input() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_lowercase()
}

fn main() {
    println!("Hello. What's your name?");

    let user_name = ask_for_input();

    println!("Hello {}", user_name);
}
