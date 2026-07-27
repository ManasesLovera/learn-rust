use std::io;
use rand::random_range;
use std::cmp::Ordering;

fn main() { println!("Guess the number!");
    let secret_number: i32 = random_range(1..=5);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess_number: i32 = match guess.trim_end().parse::<i32>() {

        Ok(num) => num,
        Err(_) => {
            println!("Error parsing guess number, using 0 as default");
            0
        }
    };


    println!("You guessed: {guess}");

    match guess_number.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win"),
    }

    println!("The correct secret number: {secret_number}");
}

