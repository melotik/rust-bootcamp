fn main() {
    let name1 = String::from("Pranav");
    let name2 = String::from("Tiago");
    
    // Both strings are now in the same scope, so the returned reference
    // from bigger_string will be valid for the println! call
    let bigger = bigger_string(&name1, &name2);
    
    println!("The bigger string is: {}", bigger);
}

fn bigger_string<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}