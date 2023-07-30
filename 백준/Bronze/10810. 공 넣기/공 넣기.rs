use std::io;

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let mut sp = input.split_ascii_whitespace();
        if let (Some(n), Some(m)) = (sp.next(), sp.next()) {
            if let (Ok(n), Ok(m)) = (n.parse::<usize>(), m.parse::<u32>()) {
                let mut arr: Vec<u32> = vec![0; n];
                for _ in 0..m {
                    input.clear();
                    if let Ok(_) = io::stdin().read_line(&mut input) {
                        let mut sp = input.split_ascii_whitespace();
                        if let (Some(i), Some(j), Some(k)) = (sp.next(), sp.next(), sp.next()) {
                            if let (Ok(i), Ok(j), Ok(k)) = (i.parse::<usize>(), j.parse::<usize>(), k.parse::<u32>()) {
                                if i > n {
                                    continue;
                                }
                                for x in i.min(n)..=j.min(n) {
                                    arr[x - 1] = k;
                                }
                            }
                        }
                    }
                }

                println!("{}", arr.iter().map(ToString::to_string).collect::<Vec<String>>().join(" "));
            }
        }
    }
}