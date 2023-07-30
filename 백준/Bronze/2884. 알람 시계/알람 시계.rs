use std::io;

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let mut sp = input.split_whitespace();
        if let (Some(h), Some(m)) = (sp.next(), sp.next()) {
            if let (Ok(h), Ok(m)) = (h.parse::<i32>(), m.parse::<i32>()) {
                let (hour, min) = if m < 45 { 
                    if h == 0 { 
                        (23, 60 - (45 - m)) 
                    } else { 
                        (h - 1, 60 - (45 - m)) 
                    }
                } else { 
                    (h, m - 45) 
                };
                println!("{hour} {min}");
            }
        }
    }
}