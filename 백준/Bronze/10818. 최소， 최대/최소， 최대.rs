use std::io::{self, Write};
fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        if let Ok(_) = input.trim().parse::<u32>() {
            input.clear();
            if let Ok(_) = io::stdin().read_line(&mut input) {
                let v = input
                    .trim()
                    .split_ascii_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect::<Vec<_>>();
                writeln!(
                    out,
                    "{} {}",
                    v.iter().min().unwrap(),
                    v.iter().max().unwrap()
                )
                .unwrap();
            }
        }
    }
}
