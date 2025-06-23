fn main() {
    // Move both names into the same scope so they live long enough
    let name1 = String::from("Pranav");
    let name2 = String::from("Tiago");

    let bigger = bigger_string(&name1, &name2);

    println!("The bigger string is: {}", bigger);
}

// Returns a reference to the longer string slice
fn bigger_string<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
