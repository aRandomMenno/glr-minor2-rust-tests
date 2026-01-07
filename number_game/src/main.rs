use std::{process::exit};

mod random;
mod guess;

fn main() {
    println!("✦ ---------------[ Raad het getal! ]--------------- ✦");
    println!("| Er is een getal tussen 1 en 32 dat je moet raden. |");
    println!("| (inclusief 1 en 32)                 Door: Menno.  |");
    println!("| Je hebt 5 pogingen om het juiste getal te raden.  |");
    println!("| Foutive pogingen als \"abc\" worden NIET meegeteld. |");
    println!("✦ ------------------------------------------------- ✦");
    // Initializeer alle variablen
    let min: i16 = 1;
    let max: i16 = 33;
    // Verkrijg een willekeurig getal
    let generated_number: i16 = random::get_number(min, max);
    let mut tries: i16 = 0;
    println!("sys://debug:random_num:{}", generated_number);
    println!("sys://guess:messages:make_a_guess");

    loop {
        tries += 1;
        println!("sys://debug:random_num:{}", tries);
        let int_guess: i16 = guess::read_input();
        if int_guess == generated_number {
            println!("sys://guess:messages:correct");
            println!("sys://guess:messages:you_got_it_in_tries:{}", tries);
            exit(0);
        }
        if tries >= 5 {
            println!("sys://guess:messages:no_more_tries");
            println!("sys://guess:messages:the_number_was:{}", generated_number);
            exit(1);
        }
        println!("sys://guess:messages:incorrect");
        if int_guess >= generated_number {
            println!("sys://guess:messages:too_high");
        }
        else if int_guess <= generated_number {
            println!("sys://guess:messages:too_low");
        }
        println!("sys://guess:messages:try_again");
    }
}