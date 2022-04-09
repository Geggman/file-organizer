use std::io;
use std::process::Command;

fn main() {
    let mut stage = 1;
    let secret_word: String = utils::strip(&mut utils::pick_random_word());
    let mut wrong_guesses: Vec<String> = Vec::new();
    let mut prompt: Vec<char> = "_".repeat(secret_word.len()).chars().collect();
    loop {
        Command::new("clear")
            .status()
            .expect("Error running 'clear'");
        utils::print_stage(stage);

        if utils::check_win(&prompt) {
            println!("YOU WON!");
            println!("The word was {}", secret_word);
            break;
        } else if stage == 7 {
            println!("YOU LOST");
            println!("The word was {}", secret_word);
            break;
        }

        println!("{}", String::from_iter(&prompt));
        println!("{:?}", wrong_guesses);
        println!("Enter a guess: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Invalid input");
        guess.trim().to_string();

        let valid: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let guess_clone: Vec<char> = guess.chars().collect();

        if guess.len() != 2
            || valid.iter().any(|&i| guess_clone.contains(&i)) == false
            || wrong_guesses.iter().any(|i| guess.contains(i)) == true
            || guess == " "
        {
            println!("Invalid input");
        } else if utils::check_right(&guess, &secret_word) {
            prompt = utils::update_prompt(&guess, &secret_word, &mut prompt);
            println!("You are right!");
        } else if !utils::check_right(&guess, &secret_word) {
            println!("{} is wrong", guess);
            wrong_guesses.push(guess.clone().trim().to_string());
            stage += 1;
        }
    }
}
