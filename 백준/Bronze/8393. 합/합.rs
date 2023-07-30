use std::io::{self};

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        if let Ok(n) = input.trim().parse::<u32>() {
            let ans = if n % 2 == 0 {
                let x = n;
                (1 + x) * (x / 2)
            } else {
                let x = n + 1;
                ((1 + x) * (x / 2)) - x
            };
            println!("{ans}");
        }
    }
}