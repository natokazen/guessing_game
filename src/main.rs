extern crate rand;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("\n --------------------- 󱎓 Welcome to Guess the Number --------------------------- ");
    print!(
        r#"
          _____                 _             _____
         / ___/_ _____ ___ ___ (_)__  ___ _  / ___/__ ___ _  ___
        / (_ / // / -_|_-<(_-</ / _ \/ _ `/ / (_ / _ `/  ' \/ -_)
        \___/\_,_/\__/___/___/_/_//_/\_, /  \___/\_,_/_/_/_/\__/
                                    /___/
        "#
    );

    const MAX_GUESS: u32 = 7;

    let secret_number = rand::random_range(1..101);
    let mut counter: u32 = 0;

    loop {
        // Check if Counter reaches MAX_GUESS to exit program
        if counter == MAX_GUESS {
            println!("\n ------------------- Game Over ------------------------\n");
            println!(
                "            You Lost 😋! Secret Number was: {}",
                secret_number
            );
            break;

            // Else countinue with the game
        } else {
            let attempts_left = MAX_GUESS - counter;

            if attempts_left == 7 {
                println!("\n\n You've got {} attempts.", MAX_GUESS);
            } else if attempts_left > 1 {
                println!("\n\n Attempts {}/{} left.", attempts_left, MAX_GUESS);
            } else {
                println!("\n\n Last attempt!");
            }

            let mut guess = String::new();

            print!("\n Enter a number: ");
            io::stdout().flush().expect("Input flush failed");

            io::stdin()
                .read_line(&mut guess)
                .expect("reading input failed");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("\n You've hit a typo! ⌨️, Please enter only numbers.");
                    continue;
                }
            };

            let distance = guess.abs_diff(secret_number);

            // if distance == 0 {
            //     // Does nothing here
            //     // Also discovered that you could leave it like to but it will still compile
            if distance <= 5 {
                println!("\n 🔥 You're boiling hot!");
            } else if distance <= 15 {
                println!("\n ☀️ You're warm!");
            } else {
                println!("\n ❄️ You're freezing cold!");
            }

            match guess.cmp(&secret_number) {
                Ordering::Greater => {
                    println!("    TOO BIG!");
                    counter += 1;
                }

                Ordering::Less => {
                    println!("    TOO SMALL!");
                    counter += 1;
                }

                Ordering::Equal => {
                    println!("\n ---------------------- You won! 🥳 ------------------------- ");
                    break;
                }
            }
        }
    }
}
