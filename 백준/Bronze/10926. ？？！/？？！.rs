use std::io;

fn main() {
    let mut inputs = String::new();
    if let Ok(_) = io::stdin().read_line(&mut inputs) {
        inputs.pop();
        println!("{inputs}??!");
    }
}