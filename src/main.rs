use std::{io::{self, Write}, cmp::Ordering};
use rand::Rng;

fn main() {
    let mut tried = 0;

    loop {
        print!("what is the maximum number you want to guess? (Default: 100): ");
        io::stdout().flush().expect("Error flushing STDOUT");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading from STDIN");

        let highernum = if user_input.trim().is_empty() {
            100
        } else {
            match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            }
        };

        print!("what is the minimum number you want to guess? (Default: 1): ");
        io::stdout().flush().expect("Error flushing STDOUT");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading from STDIN");

        let lowernum = if user_input.trim().is_empty() {
            1
        } else {
            match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            }
        };

        println!("guess the number!");
        println!("the number is between {} and {}", lowernum, highernum);

        let secret_number = rand::thread_rng().gen_range(lowernum..=highernum);

        loop {
            print!("enter your guess: ");
            io::stdout().flush().expect("Error flushing STDOUT");

            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Error reading from STDIN");

            let user_input: i32 = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {}", user_input);

            match user_input.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too small!");
                    tried += 1;
                },
                Ordering::Greater => {
                    println!("Too big!");
                    tried += 1;
                },
                Ordering::Equal => {
                    println!("You win!");
                    println!("You tried {} times", tried);
                    tried = 0;
                    break;
                }
            }
        }
        
        print!("Do you want to play again? (y/n): ");
        io::stdout().flush().expect("Error flushing STDOUT");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading from STDIN");

        if user_input.trim().to_lowercase() == "n" {
            break;
        } else {
            continue;
        }
    }
}
