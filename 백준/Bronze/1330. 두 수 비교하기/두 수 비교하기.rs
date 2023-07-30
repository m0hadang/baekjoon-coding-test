fn main() {
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        let mut sp = input.split_whitespace();
        if let (Some(a), Some(b)) = (sp.next(), sp.next()) {
            if let (Ok(a), Ok(b)) = (a.parse::<i32>(), b.parse::<i32>()) {
                match a.cmp(&b) {
                    std::cmp::Ordering::Less => println!("<"),
                    std::cmp::Ordering::Equal => println!("=="),
                    std::cmp::Ordering::Greater => println!(">"),
                }
            }
        }
    }
}