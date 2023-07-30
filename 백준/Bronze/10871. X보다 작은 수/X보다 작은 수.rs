use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let mut sp = input.split_ascii_whitespace();
        if let (Some(n), Some(x)) = (sp.next(), sp.next()) {
            if let (Ok(_), Ok(x)) = (n.parse::<u32>(), x.parse::<u32>()) {
                input.clear();
                if let Ok(_) = io::stdin().read_line(&mut input) {
                    write!(
                        out,
                        "{}",
                        input
                            .split_ascii_whitespace()
                            .filter_map(|y| y.parse::<u32>().ok())
                            .filter(|&y| y < x)
                            .map(|y| y.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                    .unwrap();
                }
            }
        }
    }
}
