use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //i can just put .to_string() right after $secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                println!("Try inserting again.")
            }
            Ordering::Greater => {
                println!("Too big!");
                println!("Try inserting again.")
            }
            Ordering::Equal => {
                println!("You win!");
                println!("The sys guessed: {}", secret_number);
                break;
            }
        }
        println!("You guessed: {}", guess);
    }
}
