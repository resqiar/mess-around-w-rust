use rand::Rng;
use std::{cmp::Ordering, io, time::Instant};

fn main() {
    let v = version(5);

    println!("== Guess the Number Game (GNG v{v}) ==");

    println!("PICK A LEVEL");

    // Pick a level here
    let level: i32 = take_level_input();

    println!("Input the number between 1 - {level} and i will tell you if it is higher or lower");

    // Generate a secret_number based on random number from 1 - n
    let secret_number = rand::thread_rng().gen_range(1..=level);

    // Steps to take for user to guess the secret number
    let mut steps: u32 = 0;

    // Duration it takes to guess the number (speedrunner loves this lmao)
    let duration_start = Instant::now();

    loop {
        let guess: i32 = take_input();

        // increment the steps by 1
        steps += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Lol too small!"),
            Ordering::Equal => {
                let duration = duration_start.elapsed();

                println!("GG! You guessed me!");
                println!("You guessed the number {secret_number} in {steps} time(s).");
                println!(
                    "It takes {:#?} for you to guess the number. is it bad?",
                    duration
                );

                break;
            }

            Ordering::Greater => println!("Lower mate!"),
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

fn take_level_input() -> i32 {
    println!("Easy = E, Normal = N, Hard = H, Unbelievable = U");

    let mut level = String::new();

    io::stdin()
        .read_line(&mut level)
        .expect("Failed to read line");

    // Get all the chars of anything typed
    let chars: Vec<char> = level.to_lowercase().chars().collect();

    // The first letter of chars
    let level = chars[0];

    let level_ratio: i32 = match level {
        'e' => {
            println!("YOU CHOOSE EASY");
            10
        }
        'n' => {
            println!("YOU CHOOSE NORMAL");
            100
        }
        'h' => {
            println!("YOU CHOOSE HARD");
            300
        }
        'u' => {
            println!("YOU CHOOSE UNBELIEVABLE!!!");
            1000
        }
        _ => {
            println!("YOU DONT PICK!!! SETTING TO DEFAULT (EASY)");
            10
        }
    };

    return level_ratio;
}

fn version(num: i32) -> i32 {
    num + 1
}
