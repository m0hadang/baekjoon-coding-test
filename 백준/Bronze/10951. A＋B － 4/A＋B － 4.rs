use std::io::{self, Write, Read};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_to_string(&mut input) {
        input.split('\n').for_each(|x| {
            let s = x
                .split_ascii_whitespace()
                .filter_map(|y| y.parse::<u32>().ok())
                .sum::<u32>();
            if s != 0 {
                writeln!(out, "{s}", ).unwrap()
            }
        });
    }
}
