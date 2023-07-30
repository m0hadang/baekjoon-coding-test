use std::io;

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let mut numbers = input.split_whitespace();
        if let (Some(a), Some(b), Some(c)) = (numbers.next(), numbers.next(), numbers.next()) {
            if let (Ok(a), Ok(b), Ok(c)) = (a.parse::<i32>(), b.parse::<i32>(), c.parse::<i32>()) {
                let ans = match (a == b, b == c, a == c) {
                    (true, true, true) => 10_000 + (a * 1_000),
                    (false, false, false) => a.max(b).max(c) * 100,
                    _ => {
                        let match_val = if a == b {
                            a
                        } else if b == c {
                            b
                        } else {
                            c
                        };
                        1_000 + (match_val * 100)
                    }
                };
                println!("{ans}");
            } else {
                println!("Error: Invalid input");
            }
        }
    }
}
