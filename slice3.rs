fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let maybe_last = slice.get(5);
    let last = if maybe_last.is_some() {
        *maybe_last.unwrap()
    } else {
        -1
    };
    /* Note the * - the precise type inside the Some is &i32,
     * which is a reference. We need to dereference this to get
     * back to a i32 value.
     */
    println!("last is {}", last);

    /* Which is long-winded, so there's a shortcut - unwrap_or will return
     * the value it is given if the Option was None.
     * The types must match up - get returns a reference.
     * so you have to make up a &i32 with &-1.
     * Finally, again use * to get the value as i32.
     */
    let last2 = *slice.get(5).unwrap_or(&-1);
    println!("last2 is {}", last2)
}
