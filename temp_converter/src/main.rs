use std::io;

// C to F: F = C*(9/5) + 32
// F to C: C = (F-32)*(5/9)

fn main() {
    println!("Do you want to convert Celsius to Fahrenheit or Fahrenheit to Celsius?");
    println!("Input C for Fahrenheit to Celsius OR F for Celsius to Fahrenheit");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read conversion choice");

    let t = String::from(choice);

    println!("You want to convert to: {t}");
    println!("what temperature would you like to convert?");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature input.");

    let temp: i32 = match temp.trim().parse() {
        Ok(temp) => temp,
        Err(_) => -1,
    };

    match t.as_str() {
        "C\n" => println!("{}", ftoc(temp)),
        "F\n" => println!("{}", ctof(temp)),
        _ => println!("t = {:?}", t),
    }
}

// F to C: C = (F-32)*5/9
fn ftoc(f: i32) -> f64 {
    ((f - 32) * 5 / 9).into()
}

// C to F: F = (C*9/5) + 32
fn ctof(c: i32) -> i32 {
    (c * 9 / 5) + 32
}