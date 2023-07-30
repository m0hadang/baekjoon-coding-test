use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        if let Ok(n) = input.trim().parse::<usize>() {
            (0..n).for_each(|i| {
                let stars: String = (0..=i).map(|_| '*').collect();
                writeln!(out, "{}", stars).unwrap();
            });
        }
    }
}