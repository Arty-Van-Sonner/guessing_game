use std::cmp::Ordering;
use std::{io};

// use rand::Rng;

fn main() {
    start_game()
}

fn exit_words() -> Box<[String]> {
    let exit_words: Vec<String> = vec![
        String::from("exit"),
        String::from("exit()"),
        String::from("ex"),
        String::from("ex()"),
    ];
    return exit_words.into_boxed_slice();
}

fn start_game() {
    println!("Guess the number!");

    let secret_number = rand::random_range(1..=100);

    // println!("The secret number is: {secret_number}");

    let mut game_continue: bool = true;

    let exit_words_box = exit_words();

    let mut attempts_count: u64 = 0;

    while game_continue {

        attempts_count += 1;
        println!("Please input your guess.");

        let mut guess: String = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = guess.trim().to_string();
        if exit_words_box.contains(&guess) {
            println!("End work!");
            return;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease type a number!\n");
                continue
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                game_continue = false;
            },
        }
        println!("");      
    }
    println!("Number of attempts: {attempts_count}");
}
