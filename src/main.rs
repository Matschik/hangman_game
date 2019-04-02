extern crate rand;

use rand::Rng;
use std::io;

fn random_number(min: i32, max: i32) -> i32 {
    let mut range = rand::thread_rng();
    return range.gen_range(&min, &max);
}

fn main() {
    println!("Welcome to Hangman game !");
    const WORDS: [&str; 9] = [
        "Feuille",
        "Trombone",
        "Gilet pare-balles",
        "Arc-en-ciel",
        "Son",
        "Casquette",
        "Mouchoir",
        "Taille",
    ];
    let nb = random_number(0, WORDS.len() as i32);

    let word_to_guess = WORDS[nb as usize].to_uppercase();
    println!("word to guess: {}", word_to_guess);
    let mut letters_found: Vec<char> = vec![];

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_len = guess.trim().len(); // Always trim user input
        if guess_len != 1 {
            continue;
        }

        for letter in word_to_guess.chars() {
            let guess_letter = guess.trim().to_uppercase().chars().next().unwrap();
            let mut is_already_found = false;

            if letter == guess_letter {
                for &letter_found in &letters_found {
                    if &letter_found == &guess_letter {
                        println!("Letter {} already found !", guess_letter);
                        is_already_found = true;
                        break;
                    }
                }
                if is_already_found {
                    break;
                }
                println!("Good job !");
                //@TODO https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position
                letters_found.push(guess_letter);
            }
        }

        // Show word state
        let mut word_state = String::new();
        for letter in word_to_guess.chars() {
            let mut is_present = false;

            match letter {
                '-' => {
                    word_state.push('-');
                    continue;
                }
                ' ' => {
                    word_state.push(' ');
                    continue;
                }
                _ => (),
            }

            for letter_found in &letters_found {
                if &letter == letter_found {
                    word_state.push(*letter_found);
                    is_present = true;
                    break;
                }
            }

            if !is_present {
                word_state.push('_');
            }
        }

        println!("{}", &word_state);
        //println!("You guessed these letters: {:?}", &letters_found);
    }
}
