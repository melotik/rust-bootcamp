
fn main() {
    let name = String::from("Rust");
    let number = 2;
    greet_times(name, number);
}

fn greet_times(name: String, times: u8) {
    for _ in 0..times {
        println!("Hello, {}!", name);
    }
    
}
