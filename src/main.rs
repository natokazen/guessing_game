extern crate rand;
use std::io::{self, Write};
use std::cmp::Ordering;

fn main() {
    println!("\n --------------------- 󱎓 Welcome to Guess the Number --------------------------- ");
    print!(r#"
          _____                 _             _____              
         / ___/_ _____ ___ ___ (_)__  ___ _  / ___/__ ___ _  ___ 
        / (_ / // / -_|_-<(_-</ / _ \/ _ `/ / (_ / _ `/  ' \/ -_)
        \___/\_,_/\__/___/___/_/_//_/\_, /  \___/\_,_/_/_/_/\__/ 
                                    /___/                        
        "#);

    let secret_number = rand::random_range(1..101);

    loop {

        let mut guess = String::new();

        print!("\n Enter a number: ");
        io::stdout().flush().expect("Input flush failed");


        io::stdin().read_line(&mut guess).expect("reading input failed");

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
