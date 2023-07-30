use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut sp = input.trim().split_ascii_whitespace();
    if let (Some(val1), Some(val2)) = (sp.next(), sp.next()) {
        let val1: String = val1.chars().rev().collect();
        let val2: String = val2.chars().rev().collect();
        if let (Ok(val1), Ok(val2)) = (val1.parse::<usize>(), val2.parse::<usize>()) {
            println!("{}", val1.max(val2));
        }
    }

    Ok(())
}
