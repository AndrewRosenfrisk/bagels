//https://inventwithpython.com/bigbookpython/project1.html
use std::io::stdin;
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    const DIGITS: u8 = 3;
    const MAX_GUESSES: u8 = 10;

    println!("Bagels, a deductive logic game.
 By Al Sweigart al@inventwithpython.com
 
 I am thinking of a {DIGITS}-digit number with no repeated digits.
 Try to guess what it is. Here are some clues:
 When I say:    That means:
   Pico         One digit is correct but in the wrong position.
   Fermi        One digit is correct and in the right position.
   Bagels       No digit is correct.
 
 For example, if the secret number was 248 and your guess was 843, the
 clues would be Fermi Pico.");

 fn get_secret_num() -> String {
    let mut numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    numbers.shuffle(&mut thread_rng());

    numbers[0..3].join("").to_string()
 }

 fn get_clues(guess: &str, secret_num: &str) -> String {
    let mut clue_vec: Vec<String> = vec![];

    clue_vec.extend(
        guess
                .chars()
                .zip(secret_num.chars())
                .map(|x| {
                    if x.0 == x.1 {
                        "Fermi".to_string()
                    } else if secret_num.contains(x.0) {
                        "Pico".to_string()
                    } else {
                        "".to_string()
                    }})
    );

    if clue_vec.join(" ").trim() == "" {
        "Bagels".to_string()
    } else {
        clue_vec.sort_unstable();

        clue_vec.join(" ").trim().to_string()
    }
 }

 fn get_game_input() -> String {
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    
    input.trim().to_string()
 }

    'game: loop {    
        let secret_num = get_secret_num();
        
        println!("I have thought of a number. You have {MAX_GUESSES} guesses to get it (guess):");

        let mut guess_count: u8 = 1;
        
        while guess_count <= MAX_GUESSES { 
            
            let guess_input: String = get_game_input();
            
            println!("Your guess #{:?} was {:?}", guess_count, guess_input);
            
            if guess_input.parse::<u16>().is_err() || guess_input.parse::<u16>().unwrap() >= 1000 {
                println!("Please guess a {DIGITS:?}-digit number, without repeating any number.");
                continue;
            }

            let clues = get_clues(&guess_input, &secret_num.to_string());

            println!("{clues}");

            guess_count += 1;

            if guess_input == secret_num {
                println!("You guessed correctly, you win!!!!");
                break;
            } else if guess_count > MAX_GUESSES {
                println!("You lost after running out of guesses :(\nThe answer was {secret_num}");
            }
        }

        println!("Do you want to play again? (y or n:)");
        let continue_prompt = get_game_input();
        
        if continue_prompt.to_ascii_lowercase() == "y" {
            continue;
        } else if continue_prompt == "n" {
            println!("Thanks for playing!");
            break 'game;
        } else {
            println!("Quitting from invalid input. Good-bye!");
            break 'game;
        }
    }
}