use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut dumy = String::new();
    io::stdin().read_line(&mut dumy).unwrap();

    let mut nums = String::new();
    io::stdin().read_line(&mut nums).unwrap();

    let mut target_num = String::new();
    io::stdin().read_line(&mut target_num).unwrap();
    let target_num = target_num.trim().parse::<i32>().unwrap();

    writeln!(
        out,
        "{}",
        nums.trim()
            .split_ascii_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .filter(|&x| x == target_num)
            .count()
    ).unwrap();
}
