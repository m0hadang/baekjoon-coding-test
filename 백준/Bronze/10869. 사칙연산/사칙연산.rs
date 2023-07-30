use std::io;

fn main() {
    let mut inputs = String::new();
    if let Ok(_) = io::stdin().read_line(&mut inputs) {
        let mut whi = inputs.split_whitespace();
        let a = whi.next().unwrap().parse::<i32>().unwrap();
        let b = whi.next().unwrap().parse::<i32>().unwrap();

        println!("{}", a + b);
        println!("{}", a - b);
        println!("{}", a * b);
        println!("{}", a / b);
        println!("{}", a % b);
    }
}