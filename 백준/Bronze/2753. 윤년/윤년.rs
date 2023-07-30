use std::io;

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        if let Ok(num) = input.trim().parse::<u32>() {
            let ans = if (num % 4 == 0 && num % 100 != 0) || (num % 400 == 0) {
                1
            } else {
                0
            };
            println!("{ans}");
        } else {
            println!("Error: Not a number");
        }
    }
}
