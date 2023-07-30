use std::io::{self};

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let input = input.trim();
        let input = input.as_bytes();
        let half_len = (input.len() / 2) as usize;
        'search: {
            for i in 0 .. half_len {
                let beg = i;
                let end = input.len() - 1 - i;
                if input[beg] != input[end] {
                    println!("0");
                    break 'search;
                }
            };
            println!("1");
        }
    }
}