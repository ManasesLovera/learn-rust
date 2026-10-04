pub fn main() {
    // panic!("crash and burn");

    // Here, we're attempting to access the 100th element of our vector, but the vector
    // has only three elements. In this situation, Rust will panic at runtime.
    let v = vec![1, 2, 3];

    v[99];
}
