use rand::{thread_rng, Rng};
use std::io::{stdin, stdout, Write};

fn get_upper_limit(input: &mut String) -> u8 {
    loop {
        input.clear();

        if let Err(e) = stdin().read_line(input) {
            println!("Something went wrong! {e}");
            print!("Try again: ");
            stdout().flush().unwrap();

            continue;
        }

        let input = input.trim();

        match input.parse::<u8>() {
            Ok(n) if n > 0 => break n,
            _ => println!("'{input}' is not valid!"),
        };

        print!("Try again: ");
        stdout().flush().unwrap();
    }
}

fn start(input: &mut String, upper_limit: u8) {
    let r = thread_rng().gen_range(0..upper_limit);
    let mut tries = 0;

    loop {
        input.clear();

        tries += 1;

        print!("Please enter your guess [0-{upper_limit}]: ");
        stdout().flush().unwrap();

        stdin().read_line(input).unwrap();

        let Ok(input) = input.trim().parse::<u8>() else {
            println!("Not a number! Try again!");
            continue;
        };

        if input == r {
            println!("Great! It took you {tries} attempts to guess the correct number!");
            break;
        } else {
            println!("Wrong number! Try again!");
        }
    }
}

fn main() {
    println!("Welcome to Rust Guessing Game!\nMade with ❤️ by JoshuaCS!\n");
    println!("The game consists of guessing a randomly selected number\n");
    print!("Let's start by selecting the upper limit: ");
    stdout().flush().unwrap();

    let input = &mut String::new();

    let upper_limit = get_upper_limit(input);

    start(input, upper_limit);
}
