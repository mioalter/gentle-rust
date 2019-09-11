// array2.rs
// read as: slice of i32

fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn main() {
    let arr = [10,20,30,40];
    // look at that &
    let res = sum(&arr);
    println!("sum {}", res);
}

/*
 * A C programmer pronounces & as 'address of';
 * a Rust programmer pronounces it 'borrow'.
 * This is going to be the key word when learning Rust.
 * Borrowing is the name given to a common pattern in programming;
 * whenever you pass something by reference (as nearly always happens
 * in dynamic languages) or pass a pointer in C.
 * Anything borrowed remains owned by the original owner.
 */
