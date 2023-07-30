use std::io::{self, Write, BufRead};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut ans = (0, 0);
    for (i, line) in (1..=9).zip(io::stdin().lock().lines()) {
        if let Ok(input) = line {
            if let Ok(n) = input.trim().parse::<u32>() {
                if n > ans.0 {
                    ans = (n, i);
                }
            }
        }
    }

    writeln!(out, "{}", ans.0).unwrap();
    writeln!(out, "{}", ans.1).unwrap();
}