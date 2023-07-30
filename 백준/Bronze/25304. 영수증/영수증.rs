fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    if let (Ok(_), Ok(_)) = (
        std::io::stdin().read_line(&mut input1), 
        std::io::stdin().read_line(&mut input2),
    ) {
        if let (Ok(total), Ok(n)) = (
            input1.trim().parse::<usize>(), 
            input2.trim().parse::<usize>(),
        ) {
            let mut sum: usize = 0;
            for _ in 1..=n {
                input2.clear();
                if let Ok(_) = std::io::stdin().read_line(&mut input2) {
                    let mut sp = input2.split_whitespace();
                    if let (Some(a), Some(b)) = (sp.next(), sp.next()) {
                        if let (Ok(a), Ok(b)) = (a.parse::<usize>(), b.parse::<usize>()) {
                            sum += a * b;
                        }
                    }
                }
            }
            println!("{}", if total == sum {"Yes"} else { "No" });
        }
    }
}