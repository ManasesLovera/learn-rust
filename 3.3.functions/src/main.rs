fn main() {

    println!("Hello, world!");

    another_function(45);
    print_labeled_measurement(5, 'h');

    let x = five();

    println!("The value is {x}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn five() -> u8 {
    5
}

fn print_labeled_measurement(value: i32, unit_label: char) {

    println!("The measurement is: {value}{unit_label}");
}

