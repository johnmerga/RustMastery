use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(
        "
        ##################
        ✨Guess the number✨
        ################## \n \n
        "
    );
    let random_num: u8 = rand::thread_rng().gen_range(1..=100);
    let trial: u8 = 5;
    let counter: u8 = 0;

    guess_game(&random_num, counter, &trial)
}

fn guess_game(random_num: &u8, mut counter: u8, trial: &u8) {
    println!("Please type your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("error occurred while reading input from command line");
    println!("you guessed :--> {:?}", guess.trim());

    match guess.trim().parse::<u8>() {
        Ok(converted_value) => {
            if counter + 1 == *trial {
                println!(
                    "
        !!!!!!!!!!!❌!Game over❌!!!!!!!!!!!!!!!! /n
        the secret number was {random_num}
        "
                );
            } else {
                counter += 1;
                match random_num.cmp(&converted_value) {
                    Ordering::Greater => {
                        println!(
                            "too small, try again \n\n\n
                    You have left {} chance
                    ",
                            trial - counter
                        );
                        guess_game(random_num, counter, trial)
                    }
                    Ordering::Less => {
                        println!(
                            "too big, try again \n\n\n
                    You have left {} chance
                    ",
                            trial - counter
                        );
                        guess_game(random_num, counter, trial)
                    }
                    Ordering::Equal => {
                        println!(
                            "********************✅congrats! you won the game!✅****************"
                        )
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("!!!!!!please only put a number between 1-100!!!!!!!\n");
            guess_game(random_num, counter, trial);
        }
    }
}
