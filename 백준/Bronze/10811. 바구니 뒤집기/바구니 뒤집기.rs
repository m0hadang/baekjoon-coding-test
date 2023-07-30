use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let mut sp = input.split_ascii_whitespace();
        if let (Some(n), Some(m)) = (sp.next(), sp.next()) {
            if let (Ok(n), Ok(m)) = (n.parse::<usize>(), m.parse::<usize>()) {
                let mut v = (0..n).map(|x| (x + 1).to_string()).collect::<Vec<String>>();
                (0..m).for_each(|_| {
                    input.clear();
                    if let Ok(_) = io::stdin().read_line(&mut input) {
                        let mut sp = input.split_ascii_whitespace();
                        if let (Some(i), Some(j)) = (sp.next(), sp.next()) {
                            if let (Ok(i), Ok(j)) = (i.parse::<usize>(), j.parse::<usize>()) {
                                v[(i - 1)..=(j - 1)].reverse();
                            }
                        }
                    }
                });
                println!("{}", v.join(" "));
            }
        }
    }
}