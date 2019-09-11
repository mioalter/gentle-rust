// It is a compile-time error to try to access arr[4]!!
fn main() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i,arr[i]);
    }
    println!("length {}", arr.len());
}
/*
Adding `println!("the 5th element is {}", arr[4])`
mioalter@mios-mbp gentle-rust $ rustc array1.rs
error: index out of bounds: the len is 4 but the index is 4
  --> array1.rs:12:39
   |
12 |     println!("the 5th element is {}", arr[4])
   |                                       ^^^^^^
   |
   = note: #[deny(const_err)] on by default

error: aborting due to previous error
*/

/*
We will come to the Rust equivalent of List soon, but arrays are not the droids you're looking for; they are fixed in size. They can be mutable (if we ask nicely) but you cannot add new elements.

Arrays are not used that often in Rust, because the type of an array includes its size.

*/
