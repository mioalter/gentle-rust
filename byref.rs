// Values can also be passed by reference.
// A reference is created by & and dereferenced by *.
// Passing by reference is important when we have a large object
// and don't wish to copy it.
fn by_ref(x: &i32) -> i32{
    *x + 1
}

// We don't need the -> (), but we can include it
// We don't need a semicolon at the end.
fn modifies(x: &mut f64) -> () {
    *x = 1.0
}

fn main() {
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1,res2);
    // 11 42

    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);
    // res is 1
}
