fn dump(name: String, arr: &[i32]) {
    println!("{} is {:?}", name, arr);
}

fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    /*
     * Without the .to_string()
     * we get
     * expected struct `std::string::String`, found reference
     * help: try using a conversion method: `"slice".to_string()`
     * expected type `std::string::String`
     * found type `&'static str`
     */
    dump("arr".to_string(), &v);

    let slice = &v[1..];
    dump("slice".to_string(), slice)
}

