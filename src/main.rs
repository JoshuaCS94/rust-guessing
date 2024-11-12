use rand::{thread_rng, Rng};
use std::io::{stdin, stdout, Write};

fn main() {
    let (stdin, mut stdout) = (stdin(), stdout());

    println!("Welcome to Rust Guessing Game!\nMade with ❤️ by JoshuaCS!\n");

    let mut input = String::new();
    let mut tries = 0;

    let mut rand = thread_rng();

    println!("The game consists of guessing a randomly selected number\n");

    print!("Let's start by selecting the upper limit: ");
    stdout.flush().unwrap();

    let upper = loop {
        input.clear();

        if let Err(e) = stdin.read_line(&mut input) {
            println!("Something went wrong! {e}");
            print!("Try again: ");
            stdout.flush().unwrap();

            continue;
        }

        let input = input.trim();

        let Ok(upper) = input.parse::<u8>() else {
            println!("'{input}' is not a number!");
            print!("Try again: ");
            stdout.flush().unwrap();

            continue;
        };

        break upper;
    };

    let r = rand.gen_range(0..upper);

    loop {
        input.clear();

        tries += 1;

        print!("Please enter your guess [0-{upper}]: ");
        stdout.flush().unwrap();

        stdin.read_line(&mut input).unwrap();

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
