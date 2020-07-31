use rand::Rng;
use std::io;

fn main() {
    let answers: [&str; 10] = [
        "SCORPION", "FLOGGING", "CROPPERS", "MIGRAINE", "FOOTNOTE", "REFINERY", "VAULTING",
        "VICARAGE", "PROTRACT", "DESCENTS",
    ];
    println!("W E L C O M E");
    println!("Difficulty (1-5)?");

    // Created a mutatable variable as a new String
    // Variables are immutable by default
    let mut difficulty = String::new(); // String is growable and UTF-8 encoded
                                        // :: means that the function is a an associated function on the type (static method)

    io::stdin()
        .read_line(&mut difficulty) // &mut passes the argument as a mutable reference, & on it's own would be immutable
        .expect("Failed to read difficulty");
    for answer in &answers {
        println!("{}", answer);
    }
    let answer_index = rand::thread_rng().gen_range(0, 11);
    let answer = String::from(answers[answer_index]);
    let answer_length = answer.len();

    let base_guesses: i32 = 10;
    let difficulty_int: i32 = difficulty.trim().parse().unwrap(); // trim needed to clean up the input from terminal
    let mut guesses_remaining = base_guesses / difficulty_int; // TODO calculate based on difficulty

    loop {
        if guesses_remaining <= 0 {
            println!("You Lose");
            break;
        }
        let mut guess = String::new();
        // TODO need input validation
        println!("Guess? ({} left)?", guesses_remaining);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");
        guess = guess.trim().to_uppercase();
        let guess_matches = find_number_char_match(&guess, &answer);
        // I know this could just be an if, but wanted to practice my match statement
        match guess_matches == answer_length {
            true => {
                println!("You win!");
                break;
            },
            false => {
                println!("{}/{} correct", guess_matches, answer_length);
                guesses_remaining -= 1;
            }
        }
        
    }
}

fn find_number_char_match(guess: &String, target: &String) -> usize {
    let mut matches: usize = 0;
    let target_len = target.len();
    if guess == target {
        return target_len;
    }
    let guess_bytes = guess.as_bytes();
    let target_bytes = target.as_bytes();

    // TODO could probs use some regex here instead
    for i in 0..target_len {
        if guess_bytes[i] == target_bytes[i] {
            matches += 1;
        }
    }

    return matches;
}
