use std::io::{self, Write, BufRead};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    if let Ok(n) = lines.next().unwrap().trim().parse::<usize>() {
        lines.take(n).enumerate().for_each(|(i, line)| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let sum: i32 = numbers.iter().sum();
            writeln!(out, "Case #{}: {} + {} = {}", i + 1, numbers[0], numbers[1], sum).unwrap();
        });
    }

    out.flush().unwrap();
}