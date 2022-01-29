use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Plase insert your guessing:");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_input.cmp(&secret_number) {
            Ordering::Less => println!("The number you insered is to small"),
            Ordering::Greater => println!("The number you insered is to big"),
            Ordering::Equal => {
                println!("Congratulations you win :)");
                break;
            }
        }
    }

    
}
