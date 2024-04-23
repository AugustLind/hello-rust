use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    loop {
        guess.clear();
        println!("What is your guess?");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Correct");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
    }
}
