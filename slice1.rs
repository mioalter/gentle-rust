// slice1.rs
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];  // open range!

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
}

/*
 * This is a neat notation which looks similar to Python slices but with a big
 * difference: a copy of the data is never made.
 * These slices all borrow their data from their arrays.
 * They have a very intimate relationship with that array,
 * and Rust spends a lot of effort to make sure that relationship
 * does not break down.
 */
