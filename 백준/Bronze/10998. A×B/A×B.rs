use std::io;

fn main() {
    let mut inputs = String::new();
    if let Ok(_) = io::stdin().read_line(&mut inputs) {
        let mut whi = inputs.split_whitespace();
        let a_val = whi.next().unwrap().parse::<i32>().unwrap();
        let b_val = whi.next().unwrap().parse::<i32>().unwrap();
        println!("{}", a_val * b_val);
    }
}