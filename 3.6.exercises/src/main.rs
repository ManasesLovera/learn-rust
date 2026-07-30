mod fibonacci;

fn main() {

    println!("Fibonacci for 23:");
    let result = fibonacci::get_nth(23);
    println!("Result: {result}");
}
