use rand::Rng;
use std::{cmp::Ordering, io, time::Instant};

fn main() {
    println!("== Guess the Number Game (GNG) ==");

    println!("Input the number and i will tell you if it is higher or lower");

    // Generate a secret_number based on random number from 1 - 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

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
