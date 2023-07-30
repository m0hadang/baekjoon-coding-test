use std::io;

fn main() {
    let mut input_hour = String::new();
    let mut input_minute = String::new();

    if let (Ok(_), Ok(_)) = (
        io::stdin().read_line(&mut input_hour),
        io::stdin().read_line(&mut input_minute),
    ) {
        let mut hour_split = input_hour.split_whitespace();
        if let (Some(hour), Some(minute)) = (hour_split.next(), hour_split.next()) {
            if let (Ok(current_hour), Ok(current_minute), Ok(cook_time)) = (
                hour.parse::<i32>(),
                minute.parse::<i32>(),
                input_minute.trim().parse::<i32>(),
            ) {
                let total_cooked_minutes = (current_minute + cook_time) % (24 * 60);
                let additional_hours = total_cooked_minutes / 60;
                let final_hour = (current_hour + additional_hours) % 24;
                let final_minute = total_cooked_minutes % 60;
                println!("{final_hour} {final_minute}");
            } else {
                println!("Error: Invalid input");
            }
        }
    }
}
