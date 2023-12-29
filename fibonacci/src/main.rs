//given the num n return the index value of the fib seq where the sequences is:
//1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144
use std::io;

fn main() {
    println!("fibonacci sequence demo");
    println!("Please input num sequence in this format: 70, 30, ... ");
    
    loop {

        let mut n: String = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
  
        println!("{}", fibonacci(n)); 
    }
}
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
