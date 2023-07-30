fn main() {
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        if let Ok(n) = input.trim().parse::<usize>() {
            for _ in 1..=n {
                input.clear();
                if let Ok(_) = std::io::stdin().read_line(&mut input) {
                    let mut sp = input.split_whitespace();
                    if let (Some(a), Some(b)) = (sp.next(), sp.next()) {
                        if let (Ok(a), Ok(b)) = (a.parse::<i32>(), b.parse::<i32>()) {
                            println!("{}", a + b);
                        }
                    }
                }
            }
        }
    }
}