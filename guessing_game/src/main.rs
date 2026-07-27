use std::io;
use rand::random_range;
use std::cmp::Ordering;

fn main() { 

    println!("Welcome to guess the number game! Predict a integer positive number from 1 to 10.\n");

    // Generate secret random number
    let secret_number: u32 = random_range(1..=10);

    println!("Please input your guess:");


    loop {
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse::<u32>() {

            Ok(num) => num,
            Err(_) => {
                println!("Error parsing guess number, you can only use integer positive numbers. Try again:");
                continue;
            }
        };

        
        println!("You guessed: {guess}");

        if guess == 0 {
            println!("0 value stops game, if you wanted to continue, only use values from 1 to 5.");
            break;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("\nYou win! Congrats!\n");
                break;
            }
        }

    };
}

