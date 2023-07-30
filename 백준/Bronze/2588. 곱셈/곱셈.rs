use std::io;

fn main() {
    let mut inputs1 = String::new();
    let mut inputs2 = String::new();

    if let (Ok(_), Ok(_)) = (
        io::stdin().read_line(&mut inputs1),
        io::stdin().read_line(&mut inputs2),
    ) {
        if let (Ok(a), Ok(b)) = (
            inputs1.trim().parse::<i32>(), 
            inputs2.trim().parse::<i32>(),
        ) {
            let mut x = b;
            while x > 0 {
                println!("{}", a * (x % 10));
                x /= 10;
            }
            println!("{}", a * b);
        }
    }
}