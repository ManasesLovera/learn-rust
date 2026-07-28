
fn main() {

    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;
    println!("The value of x is {x}");


    // Variables

    /* Integers
    
    Length | signed | unsigned
    
    8-bit     i8        u8
    16-bit    i16       u16
    32-bit    i32       u32
    64-bit    i64       u64
    128-bit   i128      u128
    
    */

    let age: u8 = 22;
    println!("My age is {age}");

    /* Integers Literal in Rust

    Number literals  |  Example

    Decimal               98_222
    Hex                   0xff
    Octal                 0o77
    Binary                0b1111_0000
    Byte (u8 only)        b'A'

    */
    
    let binary_value: u8 = 0b0000_0011;
    println!("The binary value is: {binary_value}");

    /* Floating-point types

    Length | syntax

    32-bit     f32
    64-bit     f64

    */

    let float: f32 = 3.15;
    println!("the floating number is: {float}");
    

    // Numeric operations
    
    // addition
    let sum = 5 + 10;
    println!("Sum: {sum}");

    // substraction
    let difference = 95.5 - 4.3;
    println!("Difference: {difference}");
    
    // multiplication
    let product = 4 * 30;
    println!("Product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("Quotient: {quotient} - Truncated: {truncated}");

    // remainder
    let remainder = 43 % 3;
    println!("43 % 3 = {remainder}");


    // Bolean
    let _t = true;
    let _f: bool = false; // with explicit type annotation
    
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c} - {z} - {heart_eyed_cat}");
}

