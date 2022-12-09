use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let lowernum = 1;

    let highernum = 100;

    let mut tried = 0;

    println!("guess the number!");

    println!("the number is between {} and {}", lowernum, highernum);

    let secret_number = rand::thread_rng().gen_range(lowernum..=highernum);

    loop {
        print!("enter your guess: ");
        let mut user_input = String::new();
        io::stdout().flush().expect("Error flushing STDOUT");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading from STDIN");
        
        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {user_input}");

        match user_input.cmp(&secret_number) {
            Ordering::Less => {println!("Too small!");
                tried = tried + 1;
            },
            Ordering::Greater => {println!("Too big!");
                tried = tried + 1;
            },
            Ordering::Equal => {
                println!("You win!");
                println!("You tried {} times", tried);
                break;
            }
        }
    }
}
