use std::io;

pub fn read_input() -> i16 {
    loop {
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Kan lijn niet lezen!");
        
        match guess.trim().parse::<i16>() {
            Ok(num) => if check_witin_range(num, 1, 32) {
                return num;
            } else {
                println!("sys://guess:error:out_of_range_try_again");
            },
            Err(_) => println!("sys://guess:error:invalid_value_try_again"),
        }
    }
}

fn check_witin_range(num: i16, min: i16, max: i16) -> bool {
    return num >= min && num <= max;
}