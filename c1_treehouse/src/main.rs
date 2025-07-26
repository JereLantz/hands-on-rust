use std::io::stdin;

fn main() {
    println!("Hello. What's your name?");

    let mut user_name = String::new();

    stdin().read_line(&mut user_name)
        .expect("Failed to read line");

    println!("Hello {user_name}");
}
