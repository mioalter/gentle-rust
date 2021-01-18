// Don't have to write `return` at the end of a function
// but you *can* use it to return early
fn sq(x: f64) -> f64 {
    x * x
}

fn abs(x: f64) -> f64 {
    if x > 0.0 { x }
    else { -x }
}

fn clamp(x: f64, low: f64, high: f64) -> f64 {
    if x >= low && x < high { x }
    else if x >= high { high }
    else { low }
}

// is u64 an unsigned 64-bit integer?
fn factorial(n: u64) -> u64 {
    if n == 0 { 1 }
    else { n * factorial(n-1) }
}

// pass in a pointer/reference, dereference it and add 1
fn by_ref(x: &i32) -> i32 {
    *x + 1
}

// If you want a function to modify one of its arguments
// you have to make the argument mutable
// since everything is immutable by default

fn by_ref_modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let res = sq(2.0);
    println!("square is {}", res);
    println!("abs -4.0 is {}", abs(-4.0));
    println!("clamped 4 in [1,3) is {}", clamp(4.0, 1.0, 3.0));
    println!("factorial(5) is {}", factorial(5));

    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2);

    let mut res3 = 0.0;
    by_ref_modifies(&mut res3);
    println!("res is {}", res3);
}
