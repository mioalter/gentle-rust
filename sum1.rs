/*
 * `for i in foo`
 * expects foo to be an iterator
 * slices get implicitly converted to iterators
 */
fn main() {
    let sum: i32  = (0..5).sum();
    println!("sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum);
}
