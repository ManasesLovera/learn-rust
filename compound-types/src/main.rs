use std::io;

fn main() {

    // Tuple type
    
    let tup: (u16, f32, u8) = (500, 6.4, 1);

    // Use of pattern matching to destructure a tuple value
    let (x, y, z) = tup;

    println!("The values in the tuple are: {x}, {y}, {z}");

    let one = tup.2;
    println!("Element at index 2 is {one}");

   
    // Array Type
    let _a = [1, 2, 3, 4, 5];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    let _a: [u8; 5] = [1, 2, 3, 4, 5];

    let _a = [3; 5]; // [3,3,3,3,3]
    

    // Array access

    let _a = [1,2,3,4,5];

    let _first = _a[0];
    let _second = _a[1];

    // Invalid array element access
    
   let _a = [1, 2, 3, 4, 5];

   println!("Please enter an array index:");
    
   let mut index = String::new();
   io::stdin()
       .read_line(&mut index)
       .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = _a[index];

    println!("The value of the element at index: {index} is {element}.");

}













