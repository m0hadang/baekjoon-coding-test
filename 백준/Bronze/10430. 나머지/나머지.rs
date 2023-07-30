use std::io;

fn main() {
    let mut inputs = String::new();
    if let Ok(_) = io::stdin().read_line(&mut inputs) {
        let mut sp = inputs.split_whitespace();
        if let (Some(a), Some(b), Some(c)) = (sp.next(), sp.next(), sp.next()) {
            if let (Ok(a), Ok(b), Ok(c)) = (a.parse::<i32>(), b.parse::<i32>(), c.parse::<i32>()) {
                println!("{}", (a + b) % c);
                println!("{}", ((a % c) + (b % c)) % c);
                println!("{}", (a * b) % c);
                println!("{}", ((a % c) * (b % c)) % c);
            }
        }
    }
}