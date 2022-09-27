use ansi_term::Color::Yellow;
use ansi_term::Style;
use rand::Rng;
use std::{cmp::Ordering, io, time::Instant};

fn main() {
    println!("== Guess the Number Game ==");

    println!("PICK A LEVEL");

    // Pick a level here
    let level: (i32, String) = take_level_input();

    // Print divider
    show_divider();

    println!(
        "Input the number between 1 - {} and i will tell you if it is higher or lower",
        level.0
    );

    // Generate a secret_number based on random number from 1 - n
    let secret_number = rand::thread_rng().gen_range(1..=level.0);

    // Steps to take for user to guess the secret number
    let mut steps: u32 = 0;

    // Duration it takes to guess the number (speedrunner loves this lmao)
    let duration_start = Instant::now();

    loop {
        // Take a user guess (input)
        let guess: i32 = take_input();

        // increment the steps by 1
        steps += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                // Duration since the game started
                let duration: u128 = duration_start.elapsed().as_millis();
                let duration: String = duration.to_string();

                // Print divider
                show_divider();

                println!("Well Played! You guessed me! \n");

                let level_str: String = level.1;

                println!("Stats for guessing number {secret_number}: ");
                println!("1. Level = {}", Yellow.bold().paint(level_str));
                println!("2. Step taken = {}", Yellow.bold().paint(steps.to_string()));
                println!("3. Time taken = {}ms", Yellow.bold().paint(duration));

                // Print divider
                show_divider();

                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}

fn take_input() -> i32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.trim().parse().expect("PLEASE GIVE ME A NUMBER!");

    return guess;
}

fn take_level_input() -> (i32, String) {
    println!("Easy = E, Normal = N, Hard = H, Unbelievable = U");

    let mut level = String::new();

    io::stdin()
        .read_line(&mut level)
        .expect("Failed to read line");

    // Get all the chars of anything typed
    let chars: Vec<char> = level.to_lowercase().chars().collect();

    // The first letter of chars
    let level = chars[0];

    let level_ratio: (i32, String) = match level {
        'e' => {
            println!("{}", Style::new().bold().paint("YOU CHOOSE EASY"));
            (10, String::from("Easy"))
        }
        'n' => {
            println!("{}", Style::new().bold().paint("YOU CHOOSE NORMAL"));
            (100, String::from("Normal"))
        }
        'h' => {
            println!("{}", Style::new().bold().paint("YOU CHOOSE HARD"));
            (500, String::from("Hard"))
        }
        'u' => {
            println!("{}", Style::new().bold().paint("YOU CHOOSE UNBELIEVABLE"));
            (10000, String::from("Unbelievable"))
        }
        _ => {
            println!(
                "{}",
                Style::new()
                    .bold()
                    .paint("YOU DONT PICK!!! SETTING TO DEFAULT (EASY)")
            );
            (10, String::from("Easy"))
        }
    };

    return level_ratio;
}

fn show_divider() {
    println!("<====================================>");
}
