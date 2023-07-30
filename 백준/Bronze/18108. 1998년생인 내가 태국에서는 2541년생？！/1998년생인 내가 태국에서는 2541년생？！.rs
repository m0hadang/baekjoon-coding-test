use std::io;

fn main() {
    let mut inputs = String::new();
    if let Ok(_) = io::stdin().read_line(&mut inputs) {
        if let Ok(num) = inputs.trim().parse::<u32>() {
            println!("{}", num - 543);
        }
    }
}