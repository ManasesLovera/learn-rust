mod fibonacci;
mod temp_converter;

fn main() {

    println!("Fibonacci for 23:");
    let result = fibonacci::get_nth(23);
    println!("Result: {result}");

    println!("Convert from Fahrenheit to Celsius: 98.6");
    let result = temp_converter::fahrenheit_to_celsius(98.6);

    println!("Result: {result}");
}
