fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);

    let mut sp = input.split_whitespace();

    let king = sp.next().unwrap().parse::<i32>().unwrap();
    let queen = sp.next().unwrap().parse::<i32>().unwrap();
    let rook = sp.next().unwrap().parse::<i32>().unwrap();
    let bishop = sp.next().unwrap().parse::<i32>().unwrap();
    let knight = sp.next().unwrap().parse::<i32>().unwrap();
    let pawn = sp.next().unwrap().parse::<i32>().unwrap();
    println!(
        "{} {} {} {} {} {}",
        1 - king,
        1 - queen,
        2 - rook,
        2 - bishop,
        2 - knight,
        8 - pawn
    );
}
