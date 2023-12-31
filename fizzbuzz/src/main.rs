use std::io;

fn fizz_buzz(n: i32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}

fn main() {
    println!("Let's play FizzBuzz!");
    println!("Enter a number and well see if it is divisible by 3, 5, or by both.");
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            //if err continue loop and ignore all errs
            Err(_) => continue,
        };

        println!("{:?}", fizz_buzz(input));
    }
}
