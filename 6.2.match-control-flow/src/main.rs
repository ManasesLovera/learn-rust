enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin = Coin::Penny;
    let value_in_cent = value_in_cents(coin);

    println!("The value in cents is: {value_in_cent}");

    let five: Option<i32> = Option::Some(5);
    let _six = plus_one(five);

    println!("Value: {}", five.unwrap_or(0));
}
