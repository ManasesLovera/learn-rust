fn main() {

    // if statement
    let number: u8 = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    // Handle multiple conditions with else if

    let number: u8 = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


    // Repetition with loops
    // loop, while, for

    
    let mut count: u8 = 0;

    loop {
    
        if count <= 10 {
        
            println!("count {count}");
            count += 1;

            continue;
        }

        break;
    }

    // Returning values from loop

    let mut counter: u8 = 0;

    let result: u8 = loop {
        
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    
    // Disambiguating with loop labels

    let mut count: u8 = 0;

    'counting_up: loop {

        println!("count {count}");

        let mut remaining: u8 = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    
    // Streaming conditional loops with while

    let mut number: u8 = 3;

    while number != 0 {

        println!("{number}");
        number -= 1;
    }
    println!("PRINTOFF!!!");

    // Looping through a collection with for
    
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }

    // Certain number of times with Range

    for number in (1..4).rev() {
         println!("{number}!");
    }
    println!("LIFTOFF!!!");

}

