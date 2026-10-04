// Recoverable errors
// Let’s call a function that returns a Result value because the function could fail.
mod file_helper;
use file_helper::create;

fn main() {
    let file_content = create::read_file_or_create("hello.txt");

    match file_content {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error: {}", e),
    }
}
