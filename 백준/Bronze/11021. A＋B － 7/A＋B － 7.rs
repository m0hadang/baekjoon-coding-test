use std::io::Write;

fn main() {
    let stdout = std::io::stdout();
    let mut out = 
        std::io::BufWriter::new(stdout.lock());
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        if let Ok(n) = input.trim().parse::<usize>() {
            (0..n).for_each(|i| {
                input.clear();
                if let Ok(_) = std::io::stdin().read_line(&mut input) {
                    let sum = input
                        .split_whitespace()
                        .filter_map(|x| x.parse::<i32>().ok())
                        .sum::<i32>();
                    writeln!(out, "Case #{}: {sum}", i + 1).unwrap();
                }
            });
        }
    }
}