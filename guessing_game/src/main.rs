//io = input/output library allows us to accpet user input.
//use is like import statment. bringing the io lib from standard lib (std)
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // ask user for input

    println!("Welcome to the guessing game!");

    println!("Guess a number between 1 - 100.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        //declare var with expected input types
        let mut guess = String::new();
        //reading the std in and passing guess as the argument to take the value of the input. expect to read it if not throw err
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //matching guess to make sure parse returns and okay result that let guess can accept
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            //if err continue loop and ignore all errs
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}