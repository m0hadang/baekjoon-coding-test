use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut input = String::new();
    loop {
        input.clear();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            match input
                .split_ascii_whitespace()
                .filter_map(|x| x.parse::<u32>().ok())
                .sum::<u32>()
            {
                0 => {
                    break;
                }
                x => {
                    writeln!(out, "{x}").unwrap();
                }
            }
        }
    }
}
