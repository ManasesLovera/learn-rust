

fn main() {
    
    let s = "hello";

    {
        let s = "hello2";
        println!("{s}");
    }

    println!("{s}");

    // ---
    
    let _s = String::from("hello");

    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{s}");


    
}
