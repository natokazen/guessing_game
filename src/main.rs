extern crate rand;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game");

    let secret_number = rand::random_range(1..101);

    loop {
        println!("Enter a number:");

        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("reading input failed");

        let guess :u32 = guess.trim().parse().expect("failed to convert into number");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win! 🥳");
                break;
            }
        }
    }

}
