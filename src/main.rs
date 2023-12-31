use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=20);

    println!("Your secret number is: {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Error reading input");

        let guess : u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => { 
                println!("Not a number");
                continue
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win"); 
                break;
            }
        }
    }
}
