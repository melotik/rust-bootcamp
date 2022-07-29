fn main() {
    let name1 = String::from("Pranav");
    let bigger = {
        let name2 = String::from("Tiago");
        let bigger = bigger_string(&name1, &name2);
        bigger;
    };
    println!("The bigger string is: {bigger}");
}

fn bigger_string<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
