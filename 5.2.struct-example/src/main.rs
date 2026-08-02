// An Example Program Using Structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");
    /* Output:

    rect1 is Rectangle { width: 30, height: 50 }
    rect1 is Rectangle {
        width: 30,
        height: 50,
    }

    */

    dbg!(&rect1);
    /* Output:

    [src/main.rs:26:5] &rect1 = Rectangle {
        width: 30,
        height: 50,
    }

    */

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
