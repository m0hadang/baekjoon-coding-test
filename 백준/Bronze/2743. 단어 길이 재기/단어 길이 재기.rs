fn main() {
    let mut str_input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut str_input) {
        println!("{}", str_input.trim().chars().count());
    }
}