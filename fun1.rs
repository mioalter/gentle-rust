// Either return x*x; (with a semicolon) or simply x*x
// You will still use return for returning early from a function.
fn sqr(x: f64) -> f64 {
    x * x
}

fn main() {
    let res = sqr(2.0);
    println!("square is {}", res);
}
