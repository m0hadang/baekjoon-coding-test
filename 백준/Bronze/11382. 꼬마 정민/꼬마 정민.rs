use std::io;

fn main() {
    let mut inputs = String::new();
    if let Ok(_) = io::stdin().read_line(&mut inputs) {
        println!(
            "{}",
            inputs
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .sum::<u64>()
        );
    }
}
