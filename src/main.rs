extern crate rand;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game");

    let secret_number = 41;

    loop {
        println!("Enter a number:");
        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess).expect("reading input failed");

        let guess :u32 = guess.trim().parse().expect("failed to convert into number");

        match guess.cmp(&secret_number) {
             Ordering::Less => println!("Too small!"),
             Ordering::Greater => println!("Too big!"),
             Ordering::Equal => { 
                 println!("You win! 🥳");
                 break;
             }
        }
    }

    
}
